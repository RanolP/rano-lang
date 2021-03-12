mod input;
mod util;

pub use input::*;
pub use util::*;

pub type Error = nom::error::Error<ParseInput>;
pub type ParseResult<T> = nom::IResult<ParseInput, T, Error>;
