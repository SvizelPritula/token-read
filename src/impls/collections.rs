use std::{
    collections::{BTreeSet, BinaryHeap, HashSet, LinkedList, VecDeque},
    hash::Hash,
    str::FromStr,
};

use crate::FromTokens;

macro_rules! impl_collect {
    ($ty:ident, $bound:tt $(+ $others:tt )*) => {
        impl<T> FromTokens for $ty<T>
        where
            T: $bound $(+ $others)*,
        {
            type Error = T::Err;

            fn from_tokens<'a, I>(tokens: I) -> Result<Self, Self::Error>
            where
                I: Iterator<Item = &'a str>,
            {
                tokens.map(|v| v.parse()).collect()
            }
        }
    };
}

impl_collect!(Vec, FromStr);
impl_collect!(LinkedList, FromStr);
impl_collect!(VecDeque, FromStr);
impl_collect!(BTreeSet, FromStr + Ord);
impl_collect!(HashSet, FromStr + Hash + Eq);
impl_collect!(BinaryHeap, FromStr + Ord);
