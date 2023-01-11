use std::io::{BufRead, BufReader, Lines, Read};

use crate::{error::ReadLineError, FromTokens, TokenReadError};

#[derive(Debug)]
pub struct TokenReader<R> {
    lines: Lines<R>,
}

impl<R: BufRead> TokenReader<R> {
    pub fn new(read: R) -> Self {
        TokenReader {
            lines: read.lines(),
        }
    }

    pub fn line<T>(&mut self) -> Result<T, TokenReadError<T::Error>>
    where
        T: FromTokens,
    {
        let line = self.line_raw()?;
        let tokens = line.split_whitespace();

        T::from_tokens(tokens).map_err(|source| TokenReadError::ParseError { source })
    }

    pub fn line_raw(&mut self) -> Result<String, ReadLineError> {
        let line = self.lines.next();
        let line = line.ok_or_else(|| ReadLineError::EndOfFile)?;
        let line = line.map_err(|source| ReadLineError::IoError { source })?;

        Ok(line)
    }
}

impl<R: Read> TokenReader<BufReader<R>> {
    pub fn from_read(read: R) -> Self {
        TokenReader {
            lines: BufReader::new(read).lines(),
        }
    }
}
