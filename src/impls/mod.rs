//! Types (currently errors) related to specific implementations of [`FromTokens`]

#[cfg(doc)]
use crate::FromTokens;

mod array;
mod collections;
mod tuple;

pub use tuple::*;
