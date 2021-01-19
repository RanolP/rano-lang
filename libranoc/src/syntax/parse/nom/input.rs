use std::{
    iter::{once, Cloned, Enumerate},
    slice::Iter,
};

use nom::{Compare, CompareResult, InputIter, InputLength, InputTake, Needed, UnspecializedInput};

use crate::syntax::Token;

#[derive(Clone, PartialEq)]
pub struct ParseInput<'a>(pub(crate) &'a [Token]);

impl<'a> UnspecializedInput for ParseInput<'a> {}

impl<'a> InputLength for ParseInput<'a> {
    #[inline]
    fn input_len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> InputIter for ParseInput<'a> {
    type Item = Token;
    type Iter = Enumerate<Self::IterElem>;
    type IterElem = Cloned<Iter<'a, Token>>;

    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.iter_elements().enumerate()
    }
    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
        self.0.iter().cloned()
    }
    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.0.iter().position(|b| predicate(b.clone()))
    }
    #[inline]
    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        if self.0.len() >= count {
            Ok(count)
        } else {
            Err(Needed::new(count - self.0.len()))
        }
    }
}

impl<'a> InputTake for ParseInput<'a> {
    #[inline]
    fn take(&self, count: usize) -> Self {
        ParseInput(&self.0[0..count])
    }
    #[inline]
    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.0.split_at(count);
        (ParseInput(suffix), ParseInput(prefix))
    }
}

impl<'a> Compare<Token> for ParseInput<'a> {
    #[inline(always)]
    fn compare(&self, t: Token) -> CompareResult {
        let pos = self.iter_elements().zip(once(t)).position(|(a, b)| a != b);

        match pos {
            Some(_) => CompareResult::Error,
            None => {
                if self.input_len() >= 1 {
                    CompareResult::Ok
                } else {
                    CompareResult::Incomplete
                }
            }
        }
    }

    #[inline(always)]
    fn compare_no_case(&self, t: Token) -> CompareResult {
        if self.iter_elements().zip(once(t)).any(|(a, b)| a != b) {
            CompareResult::Error
        } else if self.input_len() < 1 {
            CompareResult::Incomplete
        } else {
            CompareResult::Ok
        }
    }
}
