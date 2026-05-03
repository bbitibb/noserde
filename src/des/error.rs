use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonError {
    UnexpectedEof,
    UnexpectedChar(char),
    ExpectedChar(char),
    ExpectedString,
    ExpectedNumber,
    ExpectedBool,
    ExpectedArray,
    UnknownField(String),
    MissingField(&'static str),
    InvalidNumber,
    InvalidEscape(char),
    InvalidUnicodeEscape,
    TrailingCharacters,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonError::UnexpectedEof => write!(f, "unexpected end of input"),
            JsonError::UnexpectedChar(ch) => write!(f, "unexpected character: {ch}"),
            JsonError::ExpectedChar(ch) => write!(f, "expected character: {ch}"),
            JsonError::ExpectedString => write!(f, "expected JSON string"),
            JsonError::ExpectedNumber => write!(f, "expected JSON number"),
            JsonError::ExpectedBool => write!(f, "expected JSON boolean"),
            JsonError::ExpectedArray => write!(f, "expected JSON array"),
            JsonError::UnknownField(field) => write!(f, "unknown field: {field}"),
            JsonError::MissingField(field) => write!(f, "missing field: {field}"),
            JsonError::InvalidNumber => write!(f, "invalid number"),
            JsonError::InvalidEscape(ch) => write!(f, "invalid escape character: {ch}"),
            JsonError::InvalidUnicodeEscape => write!(f, "invalid unicode escape"),
            JsonError::TrailingCharacters => write!(f, "trailing characters after JSON value"),
        }
    }
}

impl std::error::Error for JsonError {}