use nom::{
    error::{ErrorKind, ParseError},
    Err, IResult, InputIter, InputTake, Slice,
};

use crate::syntax::{parse::nom::ParseInput, Token, TokenKind};

pub use ::nom::{
    branch::alt,
    combinator::{all_consuming, cut, map, opt},
    multi::{fold_many0, many0, separated_list0, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
};

#[inline(always)]
pub fn err_kind<T, Error: ParseError<ParseInput>>(
    i: ParseInput,
    kind: ErrorKind,
) -> IResult<ParseInput, T, Error> {
    Err(Err::Error(Error::from_error_kind(i, kind)))
}

#[inline(always)]
pub fn err_tag<T, Error: ParseError<ParseInput>>(i: ParseInput) -> IResult<ParseInput, T, Error> {
    err_kind(i, ErrorKind::Tag)
}

pub fn satisfy<F, Error: ParseError<ParseInput>>(
    cond: F,
) -> impl Fn(ParseInput) -> IResult<ParseInput, Token, Error>
where
    F: Fn(&Token) -> bool,
{
    move |i| match (i).iter_elements().next().map(|t| {
        let b = cond(&t);
        (t, b)
    }) {
        Some((t, true)) => Ok((i.slice(1..), t)),
        _ => err_kind(i, ErrorKind::Satisfy),
    }
}

pub fn any<Error: ParseError<ParseInput>>(i: ParseInput) -> IResult<ParseInput, Token, Error> {
    match i.slice_index(1) {
        Ok(index) => {
            let (i, part) = i.take_split(index);
            Ok((i, part.tokens[0].clone()))
        }
        Err(_needed) => err_kind(i, ErrorKind::Eof),
    }
}

pub fn tag<Error: ParseError<ParseInput>>(
    tag: TokenKind,
) -> impl Fn(ParseInput) -> IResult<ParseInput, Token, Error> {
    move |i| match i.iter_elements().next().map(|t| {
        let b = t.kind == tag;
        (t, b)
    }) {
        Some((t, true)) => Ok((i.slice(1..), t)),
        _ => err_kind(i, ErrorKind::Tag),
    }
}
