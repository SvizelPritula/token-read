use std::io;
use thiserror::Error;

#[cfg(doc)]
use crate::reader::TokenReader;

/// An error returned from [`TokenReader::line`].
#[derive(Error, Debug)]
pub enum ReadTokensError<E> {
    #[error("input error")]
    IoError { source: io::Error },
    #[error("unexpected end of file")]
    EndOfFile,
    #[error("failed to parse line of tokens: \"{line}\"")]
    ParseError { source: E, line: String },
}

/// An error returned from [`TokenReader::line_raw`].
#[derive(Error, Debug)]
pub enum ReadLineError {
    #[error("input error")]
    IoError { source: io::Error },
    #[error("unexpected end of file")]
    EndOfFile,
}

/// An error returned when parsing a constant amount of tokens.
///
/// This error can be returned in two situations:
///
/// * There are too many or to few token
/// * Any of the tokens fail to parse
#[derive(Error, Debug)]
pub enum ParseTokenPatternError<E> {
    #[error("failed to parse token")]
    ParseError { source: E },
    #[error("got more than {expected} tokens")]
    TooManyTokens { expected: usize },
    #[error("got {real} tokens, expected {expected}")]
    TooFewTokens { real: usize, expected: usize },
}

impl<E> From<ReadLineError> for ReadTokensError<E> {
    fn from(value: ReadLineError) -> Self {
        match value {
            ReadLineError::IoError { source } => ReadTokensError::IoError { source },
            ReadLineError::EndOfFile => ReadTokensError::EndOfFile,
        }
    }
}
