use std::str::FromStr;

use arrayvec::ArrayVec;

use crate::{FromTokens, TokenPatternParseError};

impl<T, const N: usize> FromTokens for [T; N]
where
    T: FromStr,
{
    type Error = TokenPatternParseError<T::Err>;

    fn from_tokens<'a, I>(tokens: I) -> Result<Self, Self::Error>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut array: ArrayVec<T, N> = ArrayVec::new();

        for token in tokens.map(|v| v.parse()) {
            let token = token.map_err(|source| TokenPatternParseError::ParseError { source })?;

            array
                .try_push(token)
                .map_err(|_| TokenPatternParseError::TooManyTokens { expected: N })?;
        }

        array
            .into_inner()
            .map_err(|e| TokenPatternParseError::TooFewTokens {
                real: e.len(),
                expected: N,
            })
    }
}
