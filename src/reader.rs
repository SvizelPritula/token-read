use std::io::{BufRead, BufReader, Lines, Read};

use crate::{error::ReadLineError, FromTokens, ReadTokensError};

#[cfg(doc)]
use std::io::Stdin;

/// This struct wraps a [`BufReader`] to allow easy parsing of whitespace delimited files.
///
/// # Examples
///
/// ```
/// # use token_read::TokenReader;
/// # use anyhow::Result;
/// #
/// # fn main() -> Result<()> {
/// let mut input = TokenReader::new("James 158000 0.58".as_bytes());
/// let (name, points, win_chance): (String, u64, f64) = input.line()?;
///
/// assert_eq!(name, "James");
/// assert_eq!(points, 158000);
/// assert_eq!(win_chance, 0.58);
/// #
/// #   Ok(())
/// # }
/// ```
///
/// ```
/// # use token_read::TokenReader;
/// # use anyhow::Result;
/// #
/// # fn main() -> Result<()> {
/// let mut input = TokenReader::new("13 8 17".as_bytes());
/// let numbers: Vec<i64> = input.line()?;
///
/// assert_eq!(numbers, vec![13, 8, 17]);
/// #
/// #   Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct TokenReader<R> {
    lines: Lines<R>,
}

impl<R: BufRead> TokenReader<R> {
    /// Creates a [`TokenReader`] from a type that implements [`BufRead`], such as [`Stdin`].
    pub fn new(read: R) -> Self {
        TokenReader {
            lines: read.lines(),
        }
    }


    /// Reads and parse a single line of whitespace delimited tokens.
    pub fn line<T>(&mut self) -> Result<T, ReadTokensError<T::Error>>
    where
        T: FromTokens,
    {
        let line = self.line_raw()?;
        let tokens = line.split_whitespace();

        T::from_tokens(tokens).map_err(|source| ReadTokensError::ParseError { source })
    }

    /// Reads a single line, unmodified.
    pub fn line_raw(&mut self) -> Result<String, ReadLineError> {
        let line = self.lines.next();
        let line = line.ok_or_else(|| ReadLineError::EndOfFile)?;
        let line = line.map_err(|source| ReadLineError::IoError { source })?;

        Ok(line)
    }
}

impl<R: Read> TokenReader<BufReader<R>> {
    /// Creates a [`TokenReader`] from a type that implements [`Read`].
    /// 
    /// This is a convenience method for wrapping the reader with [`BufReader`].
    pub fn from_read(read: R) -> Self {
        TokenReader {
            lines: BufReader::new(read).lines(),
        }
    }
}
