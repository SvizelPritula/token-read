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

#[cfg(test)]
mod tests {
    use std::collections::{BTreeSet, BinaryHeap, HashSet, LinkedList, VecDeque};

    use crate::TokenReader;

    macro_rules! impl_colletion_test {
        ($test:ident, $container:ident) => {
            #[test]
            fn $test() {
                let mut input = TokenReader::new("41 42 43".as_bytes());

                let real: $container<u8> = input.line().unwrap();
                let expected: $container<u8> = (41..=43).collect();

                assert_eq!(real, expected);
            }
        };
    }

    impl_colletion_test!(reads_into_vec, Vec);
    impl_colletion_test!(reads_into_linked_list, LinkedList);
    impl_colletion_test!(reads_into_vec_deque, VecDeque);
    impl_colletion_test!(reads_into_btree_set, BTreeSet);
    impl_colletion_test!(reads_into_hash_set, HashSet);

    #[test]
    fn reads_into_binary_heap() {
        let mut input = TokenReader::new("41 42 43".as_bytes());

        let real: BinaryHeap<u8> = input.line().unwrap();
        let real: Vec<u8> = real.into_iter().collect();

        let expected: Vec<u8> = (41..=43).rev().collect();

        assert_eq!(real, expected);
    }
}
