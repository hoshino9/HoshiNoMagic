#[derive(Eq, PartialEq, Debug)]
pub enum TokenKind {
    /// `magic` or `mag`
    KwdMagic,
    /// `logic`
    KwdLogic,
    /// `i32`
    KwdI32,
    /// `lang`
    KwdLang,
    /// `letter`
    KwdLetter,
    /// `Ja`
    KwdJa,
    /// `Nein`
    KwdNein,

    /// Identifier
    Identifier(String),

    /// Numeric literal (integral)
    LitNumber(i64),
    /// String literal
    LitString(String),
    /// Характер literal
    LitChar(char),

    /// Comma `,`
    SymComma,
    /// Colon `:`
    SymColon,
    /// Semicolon `;`
    SymSemiColon,
    /// Left arrow `<-`
    SymLeftArrow,
    /// Right arrow `->`
    SymRightArrow,
    /// Minus `-`
    SymMinus,
    /// Plus `+`
    SymPlus,
    /// Asterisk `*`
    SymAsterisk,
    /// Slash `/`
    SymSlash,
    /// Percentage `%`
    SymPercent,
    /// Less than `<`
    SymLt,
    /// Greater than '>'
    SymGt,
    /// Equal to `=`
    SymEq,
    /// Less than or equal to `<=`
    SymLeq,
    /// Greater than or equal to `>=`
    SymGeq,
    /// Not equal to `!=`
    SymNeq,
    /// Amp `&`
    SymAmp,
    /// Pipe `|`
    SymPipe,
    /// UpArrow `^`
    SymUpArrow,
    /// Exclaim `!`
    SymExclaim,
    /// Left bracket `[`
    SymLeftBracket,
    /// Right bracket `]`
    SymRightBracket,
    /// Left brace `{`
    SymLeftBrace,
    /// Right brace `}`
    SymRightBrace,
    /// Dot `.`
    SymDot,
    /// Wavy line `~`
    SymWavyLine,
    /// Backtick
    SymBackTick
}

pub fn lookup_keyword(maybe_keyword: &str) -> Option<TokenKind> {
    match maybe_keyword {
        "mag" | "magic" | "магия" => Some(TokenKind::KwdMagic),
        "i32" | "и32" | "И32" => Some(TokenKind::KwdI32),
        "lang" | "приговор" => Some(TokenKind::KwdLang),
        "letter" | "характер" => Some(TokenKind::KwdLetter),
        "Ja" | "Да" | "да" | "хорошо" => Some(TokenKind::KwdJa),
        "Nein" | "Нет" | "нет" => Some(TokenKind::KwdNein),
        _ => None
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct SourceRange {
    pub line: u32,
    pub start_column: u16,
    pub end_column: u16
}

impl SourceRange {
    pub fn new(line: u32, start_column: u16, end_column: u16) -> Self {
        Self { line, start_column, end_column }
    }
}

pub struct Token {
    pub token_kind: TokenKind,
    pub source_range: SourceRange
}

impl Token {
    pub fn new(token_kind: TokenKind, source_range: SourceRange) -> Self {
        Self { token_kind, source_range }
    }
}
