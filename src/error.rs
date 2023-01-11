use std::io;
use thiserror::Error;

#[cfg(doc)]
use crate::reader::TokenReader;

/// An error returned from [`TokenReader::line`].
#[derive(Error, Debug)]
pub enum TokenReadError<E> {
    #[error("input error")]
    IoError { source: io::Error },
    #[error("unexpected end of file")]
    EndOfFile,
    #[error("failed to parse tokens")]
    ParseError { source: E },
}

/// An error returned from [`TokenReader::line_raw`].
#[derive(Error, Debug)]
pub enum ReadLineError {
    #[error("input error")]
    IoError { source: io::Error },
    #[error("unexpected end of file")]
    EndOfFile
}

/// An error returned when parsing a constant amount of tokens.
/// 
/// This error can be returned in two situations:
/// 
/// * There are too many or to few token
/// * Any of the tokens fail to parse 
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
