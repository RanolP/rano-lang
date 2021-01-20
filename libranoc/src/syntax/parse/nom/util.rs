use nom::{
    error::{ErrorKind, ParseError},
    Compare, CompareResult, Err, IResult, InputIter, InputTake,
};

use crate::syntax::{parse::nom::ParseInput, Token};

pub use ::nom::{
    branch::alt,
    combinator::{all_consuming, map, opt},
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
};

#[inline(always)]
pub fn err_kind<'a, T, Error: ParseError<ParseInput<'a>>>(
    s: ParseInput<'a>,
    kind: ErrorKind,
) -> IResult<ParseInput<'a>, T, Error> {
    Err(Err::Error(Error::from_error_kind(s, kind)))
}

#[inline(always)]
pub fn err_tag<'a, T, Error: ParseError<ParseInput<'a>>>(
    s: ParseInput<'a>,
) -> IResult<ParseInput<'a>, T, Error> {
    err_kind(s, ErrorKind::Tag)
}

pub fn any<'a, Error: ParseError<ParseInput<'a>>>(
    s: ParseInput<'a>,
) -> IResult<ParseInput, &'a Token, Error> {
    match s.slice_index(1) {
        Ok(index) => {
            let (s, part) = s.take_split(index);
            Ok((s, &part.0[0]))
        }
        Err(_needed) => Err(Err::Error(Error::from_error_kind(s, ErrorKind::Eof))),
    }
}

pub fn tag<'a, Error: ParseError<ParseInput<'a>>>(
    tag: Token,
) -> impl Fn(ParseInput<'a>) -> IResult<ParseInput<'a>, &'a Token, Error> {
    move |i: ParseInput| match i.compare(tag.clone()) {
        CompareResult::Ok => {
            let (s, part) = i.take_split(1);
            Ok((s, &part.0[0]))
        }
        _ => Err(Err::Error(Error::from_error_kind(i, ErrorKind::Tag))),
    }
}
