use std::{io::BufRead, marker::PhantomData};

use crate::{FromTokens, LineCount, ReadTokensError, TokenReader};

/// An iterator returned from [`TokenReader::take`].
#[derive(Debug)]
pub struct Take<'a, T, R, S> {
    reader: &'a mut TokenReader<R>,
    remaining: S,
    _phantom: PhantomData<T>,
}

impl<'a, T, R, S> Take<'a, T, R, S>
where
    R: BufRead,
    T: FromTokens,
    S: LineCount,
{
    /// Creates a new [`Take`] iterator. It's recommended to use [`TokenReader::take`] instead.
    pub fn new(reader: &'a mut TokenReader<R>, count: S) -> Self {
        Take {
            reader,
            remaining: count,
            _phantom: PhantomData::default(),
        }
    }
}

impl<'a, T, R, S> Iterator for Take<'a, T, R, S>
where
    R: BufRead,
    T: FromTokens,
    S: LineCount,
{
    type Item = Result<T, ReadTokensError<T::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.remaining.empty() {
            self.remaining.decrement();

            Some(self.reader.line())
        } else {
            None
        }
    }
}
