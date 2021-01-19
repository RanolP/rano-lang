mod input;
mod util;

pub use input::*;
pub use util::*;

pub type Error<'a> = nom::error::Error<ParseInput<'a>>;
pub type ParseResult<'a, T> = nom::IResult<ParseInput<'a>, T, Error<'a>>;
pub type ParseResultStd<'a, T> = Result<T, nom::Err<Error<'a>>>;
