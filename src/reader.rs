use std::io::{BufRead, BufReader, Lines, Read};

use crate::{FromTokens, TokenReadError};

#[derive(Debug)]
pub struct TokenReader<R> {
    read: Lines<R>,
}

impl<R: BufRead> TokenReader<R> {
    pub fn new(read: R) -> Self {
        TokenReader { read: read.lines() }
    }

    pub fn line<T>(&mut self) -> Result<T, TokenReadError<T::Error>>
    where
        T: FromTokens,
    {
        let line = self.read.next();
        let line = line.ok_or_else(|| TokenReadError::EndOfFile)?;
        let line = line.map_err(|source| TokenReadError::IoError { source })?;

        let tokens = line.split_whitespace();

        T::from_tokens(tokens).map_err(|source| TokenReadError::ParseError { source })
    }
}

impl<R: Read> TokenReader<BufReader<R>> {
    pub fn from_read(read: R) -> Self {
        TokenReader {
            read: BufReader::new(read).lines(),
        }
    }
}
