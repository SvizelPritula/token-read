use std::str::SplitWhitespace;

use crate::FromTokens;

pub trait AsTokens<'a> {
    type Iter: Iterator<Item = &'a str>;

    fn as_tokens(&'a self) -> Self::Iter;

    fn parse_tokens<T>(&'a self) -> Result<T, T::Error>
    where
        T: FromTokens,
    {
        T::from_tokens(self.as_tokens())
    }
}

impl<'a> AsTokens<'a> for str {
    type Iter = SplitWhitespace<'a>;

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
