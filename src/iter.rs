use std::{io::BufRead, marker::PhantomData};

use crate::{FromTokens, ReadTokensError, TokenReader};

#[derive(Debug)]
pub struct Take<'a, T, R> {
    reader: &'a mut TokenReader<R>,
    remaining: usize,
    _phantom: PhantomData<T>,
}

/// The iterator returned from [`TokenReader::take`].
impl<'a, T, R> Take<'a, T, R>
where
    R: BufRead,
    T: FromTokens,
{
    pub fn new(reader: &'a mut TokenReader<R>, count: usize) -> Self {
        Take {
            reader,
            remaining: count,
            _phantom: PhantomData::default(),
        }
    }
}

impl<'a, T, R> Iterator for Take<'a, T, R>
where
    R: BufRead,
    T: FromTokens,
{
    type Item = Result<T, ReadTokensError<T::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            self.remaining -= 1;

            Some(self.reader.line())
        } else {
            None
        }
    }
}
