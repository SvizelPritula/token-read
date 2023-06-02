use std::str::FromStr;

use thiserror::Error;

use crate::{FromTokens, ParseTokenPatternError};

macro_rules! impl_tuple {
    ($len:literal, $error_name:ident; $($index:literal, $success_type:ident, $field_name:ident, $error_type:ident, $error_variant:ident;)*) => {
        impl<$($success_type: FromStr),*> FromTokens for ($($success_type,)*)
        {
            type Error = ParseTokenPatternError<$error_name<$($success_type::Err),*>>;

            fn from_tokens<'a, I>(mut tokens: I) -> Result<Self, Self::Error>
            where
                I: Iterator<Item = &'a str>,
            {
                $(
                    let $field_name = tokens
                        .next()
                        .ok_or_else(|| ParseTokenPatternError::TooFewTokens {
                            real: $index,
                            expected: $len,
                        })?
                        .parse()
                        .map_err(|source| ParseTokenPatternError::ParseError {
                            source: $error_name::$error_variant { source },
                        })?;
                )*

                match tokens.next() {
                    Some(_) => Err(ParseTokenPatternError::TooManyTokens { expected: $len }),
                    None => Ok(($($field_name,)*)),
                }
            }
        }

        /// This enum combines all errors that can occur when parsing an tuple.
        #[derive(Error, Debug)]
        pub enum $error_name<$($error_type),*> {
            $(
                #[error(transparent)]
                $error_variant { source: $error_type },
            )*
        }
    };
}

include!(concat!(env!("OUT_DIR"), "/tuple_calls.rs"));

#[cfg(test)]
mod tests {
    use crate::{ParseTokenPatternError, ReadTokensError, TokenReader};

    #[test]
    fn reads_single_value() {
        let mut input = TokenReader::new("true".as_bytes());
        let (value,): (bool,) = input.line().unwrap();

        assert!(value);
    }

    #[test]
    fn reads_multiple_values() {
        let mut input = TokenReader::new("-1 2.5 test".as_bytes());
        let (a, b, c): (i32, f64, String) = input.line().unwrap();

        assert_eq!(a, -1);
        assert_eq!(b, 2.5);
        assert_eq!(c, "test");
    }

    #[test]
    fn reads_empty_tuple() {
        let mut input = TokenReader::new("\n".as_bytes());
        let _: () = input.line().unwrap();
    }

    #[test]
    fn returns_error_on_too_many_elements() {
        let mut input = TokenReader::new("1 2 3".as_bytes());
        let result = input.line::<(u8, u8)>();

        assert!(matches!(
            result,
            Err(ReadTokensError::ParseError {
                source: ParseTokenPatternError::TooManyTokens { expected: 2 },
                ..
            })
        ));
    }

    #[test]
    fn returns_error_on_too_few_elements() {
        let mut input = TokenReader::new("10".as_bytes());
        let result = input.line::<(u8, u8)>();

        assert!(matches!(
            result,
            Err(ReadTokensError::ParseError {
                source: ParseTokenPatternError::TooFewTokens {
                    expected: 2,
                    real: 1
                },
                ..
            })
        ));
    }
}
