use std::str::FromStr;

use thiserror::Error;

use crate::{FromTokens, TokenPatternParseError};

macro_rules! impl_tuple {
    ($len:literal, $error_name:ident; $($index:literal, $success_type:ident, $field_name:ident, $error_type:ident, $error_variant:ident;)*) => {
        impl<$($success_type: FromStr),*> FromTokens for ($($success_type,)*)
        {
            type Error = TokenPatternParseError<$error_name<$($success_type::Err),*>>;

            fn from_tokens<'a, I>(mut tokens: I) -> Result<Self, Self::Error>
            where
                I: Iterator<Item = &'a str>,
            {
                $(
                    let $field_name = tokens
                        .next()
                        .ok_or_else(|| TokenPatternParseError::TooFewTokens {
                            real: $index,
                            expected: $len,
                        })?
                        .parse()
                        .map_err(|source| TokenPatternParseError::ParseError {
                            source: $error_name::$error_variant { source },
                        })?;
                )*

                match tokens.next() {
                    Some(_) => Err(TokenPatternParseError::TooManyTokens { expected: $len }),
                    None => Ok(($($field_name,)*)),
                }
            }
        }

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
