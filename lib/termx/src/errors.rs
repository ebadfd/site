use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("EOF")]
    EOF,
    #[error("{0} unclosed string literal")]
    UnclosedStringLiteral(String),
    #[error("unknown lexer error")]
    Unknown,
    #[error("lexer not implemented")]
    NotImplemented,
}

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("EOF")]
    EOF,
    #[error("{0} unexpected token")]
    UnexpectedToken(String),
    #[error("parser implemented")]
    NotImplemented,
    #[error("lexer error {0}")]
    LexerError(String),
}
