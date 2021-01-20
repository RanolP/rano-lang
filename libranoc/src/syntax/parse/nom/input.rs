use std::{
    iter::{once, Cloned, Enumerate},
    ops::RangeFrom,
    slice::Iter,
};

use nom::{
    Compare, CompareResult, InputIter, InputLength, InputTake, Needed, Slice, UnspecializedInput,
};

use crate::syntax::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseInput<'a> {
    pub(crate) tokens: &'a [Token],
    pub(crate) binding_power: u8,
}

impl<'a> ParseInput<'a> {
    pub(crate) fn new(tokens: &'a [Token]) -> Self {
        ParseInput {
            tokens,
            binding_power: 0,
        }
    }

    pub(crate) fn with_binding_power(&self, binding_power: u8) -> Self {
        ParseInput {
            tokens: &self.tokens,
            binding_power,
        }
    }
}

impl<'a> UnspecializedInput for ParseInput<'a> {}

impl<'a> InputLength for ParseInput<'a> {
    #[inline]
    fn input_len(&self) -> usize {
        self.tokens.len()
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
        self.tokens.iter().cloned()
    }
    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.tokens.iter().position(|b| predicate(b.clone()))
    }
    #[inline]
    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        if self.tokens.len() >= count {
            Ok(count)
        } else {
            Err(Needed::new(count - self.tokens.len()))
        }
    }
}

impl<'a> InputTake for ParseInput<'a> {
    #[inline]
    fn take(&self, count: usize) -> Self {
        ParseInput {
            tokens: &self.tokens[0..count],
            binding_power: self.binding_power,
        }
    }
    #[inline]
    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.tokens.split_at(count);
        (
            ParseInput {
                tokens: suffix,
                binding_power: self.binding_power,
            },
            ParseInput {
                tokens: prefix,
                binding_power: self.binding_power,
            },
        )
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

impl<'a> Slice<RangeFrom<usize>> for ParseInput<'a> {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        ParseInput {
            tokens: &self.tokens[range],
            binding_power: self.binding_power,
        }
    }
}
