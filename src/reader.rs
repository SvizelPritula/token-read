use std::io::{BufRead, BufReader, Lines, Read};

use crate::{FromTokens, LineCount, ReadLineError, ReadTokensError, Take};

#[cfg(doc)]
use std::io::Stdin;

/// This struct wraps a [`BufReader`] to allow easy parsing of whitespace delimited files.
#[derive(Debug)]
pub struct TokenReader<R> {
    lines: Lines<R>,
}

impl<R: BufRead> TokenReader<R> {
    /// Creates a [`TokenReader`] from a type that implements [`BufRead`], such as [`Stdin`].
    pub fn new(buf_read: R) -> Self {
        TokenReader {
            lines: buf_read.lines(),
        }
    }

    /// Reads and parses a single line of whitespace delimited tokens.
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
    pub fn line<T>(&mut self) -> Result<T, ReadTokensError<T::Error>>
    where
        T: FromTokens,
    {
        let line = self.line_raw()?;
        let tokens = line.split_whitespace();

        T::from_tokens(tokens).map_err(|source| ReadTokensError::ParseError { source, line })
    }

    /// Reads a single line, unmodified.
    ///
    /// # Example
    ///
    /// ```
    /// # use token_read::TokenReader;
    /// # use anyhow::Result;
    /// #
    /// # fn main() -> Result<()> {
    /// let mut input = TokenReader::new("1. Write a parsing library in Rust.\n2. ???\n3. Profit!".as_bytes());
    /// let line = input.line_raw()?;
    ///
    /// assert_eq!(line, "1. Write a parsing library in Rust.");
    /// #
    /// #   Ok(())
    /// # }
    /// ```
    pub fn line_raw(&mut self) -> Result<String, ReadLineError> {
        let line = self.lines.next();
        let line = line.ok_or_else(|| ReadLineError::EndOfFile)?;
        let line = line.map_err(|source| ReadLineError::IoError { source })?;

        Ok(line)
    }

    /// Creates an iterator that reads and parses a specific number of lines.
    ///
    /// The line count must be an [`usize`].
    /// You can also use [`TokenReader::take_count`], which allows other types.
    ///
    /// # Example
    ///
    /// ```
    /// # use token_read::TokenReader;
    /// # use anyhow::Result;
    /// #
    /// # fn main() -> Result<()> {
    /// let mut input = TokenReader::new("1 a\n2 b\n3 c".as_bytes());
    /// let lines: Vec<(u64, char)> = input.take(3).collect::<Result<_, _>>()?;
    ///
    /// assert_eq!(lines, vec![(1, 'a'), (2, 'b'), (3, 'c')]);
    /// #
    /// #   Ok(())
    /// # }
    /// ```
    pub fn take<'a, T>(&'a mut self, count: usize) -> Take<'a, T, R, usize>
    where
        T: FromTokens,
    {
        Take::new(self, count)
    }

    /// Like [`TokenReader::take`], but can use non-[`usize`] counts.
    ///
    /// This method can use any type implementing [`LineCount`] as the element count, like [`u32`].
    /// It can be used to process more than 2^32 lines on 32-bit systems.
    ///
    /// # Example
    ///
    /// ```
    /// # use token_read::TokenReader;
    /// # use anyhow::Result;
    /// #
    /// # fn main() -> Result<()> {
    /// let mut input = TokenReader::new("1 a\n2 b\n3 c".as_bytes());
    /// let lines: Vec<(u64, char)> = input.take_count(3u64).collect::<Result<_, _>>()?;
    ///
    /// assert_eq!(lines, vec![(1, 'a'), (2, 'b'), (3, 'c')]);
    /// #
    /// #   Ok(())
    /// # }
    /// ```
    pub fn take_count<'a, T, S>(&'a mut self, count: S) -> Take<'a, T, R, S>
    where
        T: FromTokens,
        S: LineCount,
    {
        Take::new(self, count)
    }
}

impl<R: Read> TokenReader<BufReader<R>> {
    /// Creates a [`TokenReader`] from a type that implements [`Read`].
    ///
    /// This is a convenience method for wrapping the reader with [`BufReader`].
    pub fn from_read(read: R) -> Self {
        TokenReader::new(BufReader::new(read))
    }
}

impl<R> From<R> for TokenReader<R>
where
    R: BufRead,
{
    /// Wraps an implementation of [`BufRead`].
    ///
    /// Identical to [`TokenReader::new`].
    fn from(value: R) -> Self {
        TokenReader::new(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ReadLineError, ReadTokensError, TokenReader};

    #[test]
    fn can_be_constructed_from_bufread() {
        let mut input = TokenReader::new("Hello".as_bytes());
        assert_eq!(input.line_raw().unwrap(), "Hello");
    }

    #[test]
    fn can_be_constructed_from_read() {
        let mut input = TokenReader::from_read("Hello".as_bytes());
        assert_eq!(input.line_raw().unwrap(), "Hello");
    }

    #[test]
    fn can_be_constructed_with_from() {
        let mut input: TokenReader<_> = "Hello".as_bytes().into();
        assert_eq!(input.line_raw().unwrap(), "Hello");
    }

    #[test]
    fn reads_raw_lines() {
        let mut input = TokenReader::new("First\nSecond\n".as_bytes());
        assert_eq!(input.line_raw().unwrap(), "First");
        assert_eq!(input.line_raw().unwrap(), "Second");
        assert!(matches!(input.line_raw(), Err(ReadLineError::EndOfFile)));
    }

    #[test]
    fn reads_single_value() {
        let mut input = TokenReader::new("13".as_bytes());
        let value: Vec<i8> = input.line().unwrap();
        assert_eq!(value, vec![13]);
    }

    #[test]
    fn reads_multiple_values() {
        let mut input = TokenReader::new("40 50 60".as_bytes());
        let value: Vec<i8> = input.line().unwrap();
        assert_eq!(value, vec![40, 50, 60]);
    }

    #[test]
    fn reads_empty_values() {
        let mut input = TokenReader::new(" ".as_bytes());
        let value: Vec<i8> = input.line().unwrap();
        assert_eq!(value, vec![]);
    }

    #[test]
    fn ignores_multiple_whitespace_characters() {
        let mut input = TokenReader::new("1\t\r    \t  7".as_bytes());
        let value: Vec<i8> = input.line().unwrap();
        assert_eq!(value, vec![1, 7]);
    }

    #[test]
    fn ignores_start_and_end() {
        let mut input = TokenReader::new(" \t123 \r".as_bytes());
        let value: Vec<i8> = input.line().unwrap();
        assert_eq!(value, vec![123]);
    }

    #[test]
    fn returns_end_of_file() {
        let mut input = TokenReader::new("5\n".as_bytes());

        let _ = input.line::<Vec<i8>>().unwrap();
        let result = input.line::<Vec<i8>>();

        assert!(matches!(result, Err(ReadTokensError::EndOfFile)));
    }

    #[test]
    fn returns_parse_error() {
        let mut input = TokenReader::new("one\n".as_bytes());

        let result = input.line::<Vec<i8>>();

        match result {
            Err(ReadTokensError::ParseError { source: _, line }) => {
                assert_eq!(line, "one");
            }
            _ => panic!("expected error, got {result:?}"),
        }
    }

    #[test]
    fn take_gets_multiple_lines() {
        let mut input = TokenReader::new("0\n1\n2\nx".as_bytes());

        for (i, value) in input.take(3).enumerate() {
            let (value,): (usize,) = value.unwrap();
            assert_eq!(value, i);
        }
    }

    #[test]
    fn take_count_gets_multiple_lines() {
        let mut input = TokenReader::new("0\n1\n2\nx".as_bytes());

        for (i, value) in input.take_count(3u64).enumerate() {
            let (value,): (usize,) = value.unwrap();
            assert_eq!(value, i);
        }
    }
}
