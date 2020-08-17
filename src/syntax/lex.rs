use std::iter::Iterator;
use std::iter::FusedIterator;
use std::iter::Peekable;
use std::collections::BTreeSet;

use lazy_static::lazy_static;

use crate::syntax::token::{Token, lookup_keyword, SourceRange, TokenKind};
use crate::diag::{Diagnostic, DiagLevel};

lazy_static! {
    static ref CYRILLIC_CHARS: BTreeSet<char> = {
        "АБВГДЕЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯабвгдеёжзийклмнопрстуфхцчшщъыьэюя"
            .chars()
            .collect::<BTreeSet::<_>>()
    };
}

fn is_cyrillic(ch: char) -> bool {
    CYRILLIC_CHARS.contains(&ch)
}

pub struct Lexer<CharIter: FusedIterator<Item = char>> {
    intern: Peekable<CharIter>,
    token_stream: Vec<Token>,
    diags: Vec<Diagnostic>,
    line: u32,
    column: u16
}

impl<CharIter: FusedIterator<Item = char>> Lexer<CharIter> {
    pub fn new(iter: CharIter) -> Self {
        Self {
            intern: iter.peekable(),
            token_stream: vec![],
            diags: vec![],
            line: 1,
            column: 1
        }
    }

    fn next_char(&mut self) -> Option<char> {
        self.intern.next().map(|ch| {
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            }
            ch
        })
    }

    fn peek_char(&mut self) -> Option<char> {
        self.intern.peek().map(|x| *x)
    }

    fn cur_char_pos(&self) -> SourceRange {
        SourceRange::new(self.line, self.column, self.column + 1)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek_char() {
            match ch {
                ' ' | '\t' | '\r' | '\n' => {
                    self.next_char();
                },
                _ => {
                    break;
                }
            }
        }
    }

    fn lex_keyword_or_magic_sym(&mut self) {
        debug_assert!(self.peek_char().unwrap().is_alphanumeric()
            || is_cyrillic(self.peek_char().unwrap()));

        let start_col = self.column;
        let mut s = String::new();
        while let Some(ch) = self.peek_char() {
            if ch.is_alphanumeric() || is_cyrillic(ch) || ch == '!' || ch == '?' {
                self.next_char();
                s.push(ch);
            } else {
                break;
            }
        }

        self.token_stream.push(Token::new(
            if let Some(token_kind) = lookup_keyword(&s) {
                token_kind
            } else {
                TokenKind::Identifier(s)
            },
            SourceRange::new(self.line, start_col, self.column)));
    }

    fn lex_number(&mut self) {
        debug_assert!(self.peek_char().unwrap().is_numeric());

        let start_col = self.column;
        let mut num = 0i64;
        while let Some(ch) = self.peek_char() {
            if ch.to_digit(10).map(|digit| {
                self.next_char();
                num *= 10;
                num += digit as i64;
            }).is_none() {
                break;
            }
        }
        self.token_stream.push(Token::new(
            TokenKind::LitNumber(num),
            SourceRange::new(self.line, start_col, self.column)
        ));
    }

    fn lex_char(&mut self) {
        debug_assert_eq!(self.peek_char().unwrap(), '\'');
        let start_col = self.column;
        self.next_char();

        let ch = match self.peek_char() {
            Some('\\') => {
                self.next_char();
                match self.peek_char() {
                    Some('n') => '\n',
                    Some('t') => '\t',
                    Some('r') => '\r',
                    Some('\'') => '\'',
                    Some('\\') => '\\',
                    Some('0') => '\0',
                    Some(sth_else) => {
                        self.diags.push(Diagnostic::new(
                            DiagLevel::Warn,
                            format!("unknown escape character: \\{}", sth_else),
                            self.cur_char_pos()));
                        sth_else
                    },
                    None => {
                        self.diags.push(Diagnostic::new(
                            DiagLevel::Error,
                            "unfinished escape character".into(),
                            self.cur_char_pos()));
                        '\\'
                    }
                }
            },
            Some(sth_else) => {
                sth_else
            },
            None => {
                self.diags.push(Diagnostic::new(
                    DiagLevel::Error,
                    "unfinished character literal".into(),
                    self.cur_char_pos()));
                '\0'
            }
        };
        self.next_char();

        if let Some('\'') = self.peek_char() {
            self.next_char();
        } else {
            self.diags.push(Diagnostic::new(
                DiagLevel::Error,
                "unclosed character literal".into(),
                SourceRange::new(self.line, start_col, self.column)));
        }

        self.token_stream.push(Token::new(TokenKind::LitChar(ch),
                                          SourceRange::new(self.line, start_col, self.column)));
    }

    fn lex_string(&mut self) {
        debug_assert_eq!(self.peek_char().unwrap(), '"');
        let start_col = self.column;
        self.next_char();

        let mut s = String::new();
        let mut closed = false;
        while let Some(ch) = self.peek_char() {
            if ch == '"' {
                self.next_char();
                closed = true;
                break;
            }

            let this_char = if ch == '\\' {
                self.next_char();
                match self.peek_char() {
                    Some('n') => '\n',
                    Some('t') => '\t',
                    Some('r') => '\r',
                    Some('0') => '\0',
                    Some('\\') => '\\',
                    Some('"') => '\"',
                    Some(sth_else) => {
                        self.diags.push(Diagnostic::new(
                            DiagLevel::Warn,
                            format!("unknown escape character: \\{}", sth_else),
                            self.cur_char_pos()));
                        sth_else
                    },
                    None => {
                        self.diags.push(Diagnostic::new(
                            DiagLevel::Error,
                            "unfinished escape character".into(),
                            self.cur_char_pos()));
                        '\\'
                    }
                }
            } else {
                ch
            };
            s.push(this_char);
            self.next_char();
        }

        if !closed {
            self.diags.push(
                Diagnostic::new(DiagLevel::Error,
                                "unclosed string".into(),
                                SourceRange::new(self.line, start_col, self.column)));
        }

        self.token_stream.push(Token::new(TokenKind::LitString(s),
                                          SourceRange::new(self.line, start_col, self.column)));
    }

    pub fn lex(&mut self) -> (Vec<Token>, Vec<Diagnostic>) {
        let mut ret_tokens = vec![];
        let mut ret_diags = vec![];
        ret_tokens.append(&mut self.token_stream);
        ret_diags.append(&mut self.diags);
        (ret_tokens, ret_diags)
    }
}

#[cfg(test)]
mod test {
    use crate::syntax::lex::Lexer;
    use crate::syntax::token::TokenKind;

    #[test]
    fn test_lex_magic_symbol() {
        let source = "Hello привет English?Русский!";
        let mut lexer = Lexer::new(source.chars());
        for _ in 0..3 {
            lexer.lex_keyword_or_magic_sym();
            lexer.skip_whitespace();
        }
        assert_eq!(lexer.token_stream.len(), 3);

        let magic_symbols = ["Hello", "привет", "English?Русский!"];
        for n in 0..3 {
            match &lexer.token_stream[n].token_kind {
                TokenKind::Identifier(id ) => {
                    assert_eq!(id, magic_symbols[n])
                },
                _ => panic!("unexpected token kind")
            }
        }
    }

    #[test]
    fn test_lex_keywords() {
        let source =
            "magic mag магия i32 и32 И32 lang приговор letter характер Ja Nein Да да Нет нет";
        let mut lexer = Lexer::new(source.chars());
        for _ in 0..16 {
            lexer.lex_keyword_or_magic_sym();
            lexer.skip_whitespace();
        }
        assert_eq!(lexer.token_stream.len(), 16);

        let identifiers =
            [TokenKind::KwdMagic, TokenKind::KwdMagic, TokenKind::KwdMagic,
                TokenKind::KwdI32, TokenKind::KwdI32, TokenKind::KwdI32,
                TokenKind::KwdLang, TokenKind::KwdLang,
                TokenKind::KwdLetter, TokenKind::KwdLetter,
                TokenKind::KwdJa, TokenKind::KwdNein,
                TokenKind::KwdJa, TokenKind::KwdJa,
                TokenKind::KwdNein, TokenKind::KwdNein];
        for n in 0..16 {
            assert_eq!(lexer.token_stream[n].token_kind, identifiers[n]);
        }
    }

    #[test]
    fn test_lex_number() {
        let source = "114 514 1919810";
        let mut lexer = Lexer::new(source.chars());
        for _ in 0..3 {
            lexer.lex_number();
            lexer.skip_whitespace();
        }
        assert_eq!(lexer.token_stream.len(), 3);

        let numbers = [114, 514, 1919810];
        for n in 0..3 {
            if let TokenKind::LitNumber(num) = lexer.token_stream[n].token_kind {
                assert_eq!(num, numbers[n]);
            } else {
                panic!("unexpected token kind");
            }
        }
    }

    #[test]
    fn test_lex_char() {
        let source = "'a' 'b' 'c' '\\t' '\\n' '\\r' '\\0' '\\\\'";
        let mut lexer = Lexer::new(source.chars());
        for _ in 0..8 {
            lexer.lex_char();
            lexer.skip_whitespace();
        }
        assert_eq!(lexer.token_stream.len(), 8);

        let chars = "abc\t\n\r\0\\".chars().collect::<Vec<_>>();
        for n in 0..8 {
            if let TokenKind::LitChar(ch) = lexer.token_stream[n].token_kind {
                assert_eq!(ch, chars[n]);
            } else {
                panic!("unexpected token kind");
            }
        }
    }

    #[test]
    fn test_lex_string() {
        let source = r#""alpha" "beta" "gamma\theta" "\n\0\r" ",\",""#;
        let mut lexer = Lexer::new(source.chars());
        for _ in 0..5 {
            lexer.lex_string();
            lexer.skip_whitespace();
        }
        assert_eq!(lexer.token_stream.len(), 5);

        let strings = ["alpha", "beta", "gamma\theta", "\n\0\r", ",\","];
        for n in 0..5 {
            if let TokenKind::LitString(str) = &lexer.token_stream[n].token_kind {
                assert_eq!(str, strings[n])
            } else {
                panic!("unexpected token kind");
            }
        }
    }
}
