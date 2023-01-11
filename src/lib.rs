mod error;
pub mod impls;
mod reader;

pub use error::{TokenPatternParseError, TokenReadError};
pub use reader::TokenReader;

pub trait FromTokens: Sized {
    type Error;

    fn from_tokens<'a, I>(tokens: I) -> Result<Self, Self::Error>
    where
        I: Iterator<Item = &'a str>;
}
