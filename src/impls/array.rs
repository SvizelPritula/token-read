use std::str::FromStr;

use arrayvec::ArrayVec;

use crate::{FromTokens, ParseTokenPatternError};

impl<T, const N: usize> FromTokens for [T; N]
where
    T: FromStr,
{
    type Error = ParseTokenPatternError<T::Err>;

    fn from_tokens<'a, I>(tokens: I) -> Result<Self, Self::Error>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut array: ArrayVec<T, N> = ArrayVec::new();

        for token in tokens.map(|v| v.parse()) {
            let token = token.map_err(|source| ParseTokenPatternError::ParseError { source })?;

            array
                .try_push(token)
                .map_err(|_| ParseTokenPatternError::TooManyTokens { expected: N })?;
        }

        array
            .into_inner()
            .map_err(|e| ParseTokenPatternError::TooFewTokens {
                real: e.len(),
                expected: N,
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::{ParseTokenPatternError, ReadTokensError, TokenReader};

    #[test]
    fn reads_array() {
        let mut input = TokenReader::new("10 11 12 13".as_bytes());
        let value: [u8; 4] = input.line().unwrap();
        assert_eq!(value, [10, 11, 12, 13]);
    }

    #[test]
    fn returns_error_on_too_many_elements() {
        let mut input = TokenReader::new("10 11 12 13 14".as_bytes());
        let result = input.line::<[u8; 4]>();

        assert!(matches!(
            result,
            Err(ReadTokensError::ParseError {
                source: ParseTokenPatternError::TooManyTokens { expected: 4 },
                ..
            })
        ));
    }

    #[test]
    fn returns_error_on_too_few_elements() {
        let mut input = TokenReader::new("10 11 12".as_bytes());
        let result = input.line::<[u8; 4]>();

        assert!(matches!(
            result,
            Err(ReadTokensError::ParseError {
                source: ParseTokenPatternError::TooFewTokens {
                    expected: 4,
                    real: 3
                },
                ..
            })
        ));
    }
}
