use std::fmt::Display;
use std::fmt::Formatter;
use crate::syntax::token::SourceRange;

pub enum DiagLevel {
    Note,
    Warn,
    Error,
    Fatal
}

impl Display for DiagLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", match self {
            DiagLevel::Note => "note",
            DiagLevel::Warn => "warning",
            DiagLevel::Error => "error",
            DiagLevel::Fatal => "fatal"
        })?;
        Ok(())
    }
}

pub struct Diagnostic {
    pub level: DiagLevel,
    pub info: String,
    pub source_range: SourceRange,
    pub fix_hint: Option<String>,
    pub fix_hint_source_range: Option<SourceRange>
}

impl Diagnostic {
    pub fn new(level: DiagLevel,
               info: String,
               source_range: SourceRange) -> Self {
        Self {
            level,
            info,
            source_range,
            fix_hint: None,
            fix_hint_source_range: None
        }
    }

    pub fn with_fix_hint(level: DiagLevel,
                         info: String,
                         source_range: SourceRange,
                         fix_hint: String,
                         fix_hint_source_range: SourceRange) -> Self {
        Self {
            level,
            info,
            source_range,
            fix_hint: Some(fix_hint),
            fix_hint_source_range: Some(fix_hint_source_range)
        }
    }
}
