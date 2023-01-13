use std::str::SplitWhitespace;

use crate::FromTokens;

/// A trait for types that can be used to create an iterator of tokens.
///
/// It can be used to easily parse strings.
pub trait AsTokens<'a> {
    type Iter: Iterator<Item = &'a str>;

    /// Creates an iterator of tokens contained in the type.
    fn as_tokens(&'a self) -> Self::Iter;

    /// Parses all tokens into an (usually inferred) type.
    ///
    /// # Example
    ///
    /// ```
    /// # use token_read::AsTokens;
    /// # use anyhow::Result;
    /// #
    /// # fn main() -> Result<()> {
    /// let (a, b): (u64, char) = "15 B".parse_tokens()?;
    ///
    /// assert_eq!(a, 15);
    /// assert_eq!(b, 'B');
    /// #
    /// #   Ok(())
    /// # }
    /// ```
    fn parse_tokens<T>(&'a self) -> Result<T, T::Error>
    where
        T: FromTokens,
    {
        T::from_tokens(self.as_tokens())
    }
}

impl<'a> AsTokens<'a> for str {
    type Iter = SplitWhitespace<'a>;

    /// Splits the string by any whitespace (including newlines).
    fn as_tokens(&'a self) -> Self::Iter {
        self.split_whitespace()
    }
}

#[cfg(test)]
mod tests {
    use crate::AsTokens;

    #[test]
    fn splits_string_at_whitespace() {
        let tokens = "0 1".as_tokens();
        let tokens: Vec<&str> = tokens.collect();
        assert_eq!(tokens, vec!["0", "1"]);
    }

    #[test]
    fn parse_tokens_parses_tokens() {
        let numbers: Vec<u8> = "1 2".parse_tokens().unwrap();
        assert_eq!(numbers, vec![1, 2]);
    }
}
