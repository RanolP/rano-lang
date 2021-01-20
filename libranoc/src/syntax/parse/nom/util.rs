use nom::{
    error::{ErrorKind, ParseError},
    Compare, CompareResult, Err, IResult, InputIter, InputTake, Slice,
};

use crate::syntax::{parse::nom::ParseInput, Token};

pub use ::nom::{
    branch::alt,
    combinator::{all_consuming, map, opt},
    multi::{fold_many0, many0, separated_list0, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
};

#[inline(always)]
pub fn err_kind<'a, T, Error: ParseError<ParseInput<'a>>>(
    i: ParseInput<'a>,
    kind: ErrorKind,
) -> IResult<ParseInput<'a>, T, Error> {
    Err(Err::Error(Error::from_error_kind(i, kind)))
}

#[inline(always)]
pub fn err_tag<'a, T, Error: ParseError<ParseInput<'a>>>(
    i: ParseInput<'a>,
) -> IResult<ParseInput<'a>, T, Error> {
    err_kind(i, ErrorKind::Tag)
}

pub fn satisfy<'a, F, Error: ParseError<ParseInput<'a>>>(
    cond: F,
) -> impl Fn(ParseInput<'a>) -> IResult<ParseInput, Token, Error>
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

pub fn any<'a, Error: ParseError<ParseInput<'a>>>(
    i: ParseInput<'a>,
) -> IResult<ParseInput, &'a Token, Error> {
    match i.slice_index(1) {
        Ok(index) => {
            let (i, part) = i.take_split(index);
            Ok((i, &part.tokens[0]))
        }
        Err(_needed) => err_kind(i, ErrorKind::Eof),
    }
}

pub fn tag<'a, Error: ParseError<ParseInput<'a>>>(
    tag: Token,
) -> impl Fn(ParseInput<'a>) -> IResult<ParseInput<'a>, &'a Token, Error> {
    move |i: ParseInput| match i.compare(tag.clone()) {
        CompareResult::Ok => {
            let (s, part) = i.take_split(1);
            Ok((s, &part.tokens[0]))
        }
        _ => err_kind(i, ErrorKind::Tag),
    }
}
