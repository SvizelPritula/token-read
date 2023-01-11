//! This crate provides for easy parsing of whitespace delimited files.
//!
//! It is primarily intended for competitive programming.
//!
//! # Sample usage
//!
//! ## Input
//!
//! ```txt
//! 42
//! Benjamin 2536000
//! 0 1 1 2 3 5 8 13 21 34
//! ```
//!
//! ## Code
//!
//! ```no_run
//! use std::io::stdin;
//! 
//! use anyhow::Result;
//! use token_read::TokenReader;
//! 
//! fn main() -> Result<()> {
//!     let mut input = TokenReader::new(stdin().lock());
//!
//!     let (n,): (usize,) = input.line()?; // Read a single value
//!     let (name, points): (String, u64) = input.line()?; // Read several values
//!     let values: Vec<u64> = input.line()?; // Read an array of values
//!     
//!     // Do some processing
//! 
//!     Ok(())
//! }
//' ```

mod error;
pub mod impls;
mod reader;

pub use error::{ReadLineError, TokenPatternParseError, TokenReadError};
pub use reader::TokenReader;

#[cfg(doc)]
use std::str::FromStr;

/// A trait for types parsable from an iterator of whitespace delimited tokens.
///
/// Implementations are provided for tuples and collections of types implementing [`FromStr`].
pub trait FromTokens: Sized {
    type Error;

    fn from_tokens<'a, I>(tokens: I) -> Result<Self, Self::Error>
    where
        I: Iterator<Item = &'a str>;
}
