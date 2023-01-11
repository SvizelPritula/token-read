use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenReadError<E> {
    #[error("input error")]
    IoError { source: io::Error },
    #[error("unexpected end of file")]
    EndOfFile,
    #[error("failed to parse tokens")]
    ParseError { source: E },
}

#[derive(Error, Debug)]
pub enum ReadLineError {
    #[error("input error")]
    IoError { source: io::Error },
    #[error("unexpected end of file")]
    EndOfFile
}

#[derive(Error, Debug)]
pub enum TokenPatternParseError<E> {
    #[error("failed to parse token")]
    ParseError { source: E },
    #[error("got more than {expected} tokens")]
    TooManyTokens { expected: usize },
    #[error("got {real} tokens, expected {expected}")]
    TooFewTokens { real: usize, expected: usize },
}

impl<E> From<ReadLineError> for TokenReadError<E> {
    fn from(value: ReadLineError) -> Self {
        match value {
            ReadLineError::IoError { source } => TokenReadError::IoError { source },
            ReadLineError::EndOfFile => TokenReadError::EndOfFile,
        }
    }
}
