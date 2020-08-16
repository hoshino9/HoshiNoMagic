use std::iter::Iterator;
use std::iter::FusedIterator;
use std::iter::Peekable;
use std::collections::BTreeSet;

use lazy_static::lazy_static;

use crate::syntax::token::{Token, lookup_keyword, SourceRange, TokenKind};
use crate::diag::Diagnostic;

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
    errors: Vec<Diagnostic>,
    line: u32,
    column: u16
}

impl<CharIter: FusedIterator<Item = char>> Lexer<CharIter> {
    pub fn new(iter: CharIter) -> Self {
        Self {
            intern: iter.peekable(),
            token_stream: vec![],
            errors: vec![],
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

    pub fn lex_number(&mut self) {
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

    pub fn lex(&mut self) -> (Vec<Token>, Vec<Diagnostic>) {
        let mut ret_tokens = vec![];
        let mut ret_errors = vec![];
        ret_tokens.append(&mut self.token_stream);
        ret_errors.append(&mut self.errors);
        (ret_tokens, ret_errors)
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
}
