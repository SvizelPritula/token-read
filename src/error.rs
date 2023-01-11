use std::io::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenPatternParseError<E> {
    #[error("failed to parse token")]
    ParseError { source: E },
    #[error("got more than {expected} tokens")]
    TooManyTokens { expected: usize },
    #[error("got {real} tokens, expected {expected}")]
    TooFewTokens { real: usize, expected: usize },
}

#[derive(Error, Debug)]
pub enum TokenReadError<E> {
    #[error("input error")]
    IoError { source: Error },
    #[error("unexpected end of file")]
    EndOfFile,
    #[error("failed to parse tokens")]
    ParseError { source: E },
}
