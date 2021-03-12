use std::{iter::Enumerate, ops::RangeFrom, vec::IntoIter};

use nom::{InputIter, InputLength, InputTake, Needed, Slice};

use crate::syntax::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseInput {
    pub(crate) tokens: Vec<Token>,
    pub(crate) binding_power: u8,
}

impl ParseInput {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        ParseInput {
            tokens,
            binding_power: 0,
        }
    }

    pub(crate) fn with_binding_power(self, binding_power: u8) -> Self {
        ParseInput {
            tokens: self.tokens,
            binding_power,
        }
    }
}

impl InputLength for ParseInput {
    #[inline]
    fn input_len(&self) -> usize {
        self.tokens.len()
    }
}

impl InputIter for ParseInput {
    type Item = Token;
    type Iter = Enumerate<Self::IterElem>;
    type IterElem = IntoIter<Token>;

    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.iter_elements().enumerate()
    }
    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
        self.tokens.clone().into_iter()
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

impl InputTake for ParseInput {
    #[inline]
    fn take(&self, count: usize) -> Self {
        ParseInput {
            tokens: self.tokens[0..count].to_vec(),
            binding_power: self.binding_power,
        }
    }
    #[inline]
    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.tokens.split_at(count);
        (
            ParseInput {
                tokens: suffix.to_vec(),
                binding_power: self.binding_power,
            },
            ParseInput {
                tokens: prefix.to_vec(),
                binding_power: self.binding_power,
            },
        )
    }
}

impl Slice<RangeFrom<usize>> for ParseInput {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        ParseInput {
            tokens: self.tokens[range].to_vec(),
            binding_power: self.binding_power,
        }
    }
}
