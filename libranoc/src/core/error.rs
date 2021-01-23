use thiserror::Error;

use crate::syntax::Span;

#[derive(Debug)]
#[repr(u16)]
pub enum ErrorCode {
    SyntaxError = 0001,
}

#[derive(Debug)]
pub enum Location {
    Eof,
    Known(Span),
}

#[derive(Debug)]
pub struct Label {
    pub location: Location,
    pub message: Option<String>,
}

#[derive(Error, Debug)]
#[error("{message}")]
pub struct Error {
    pub code: ErrorCode,
    pub message: String,
    pub labels: Vec<Label>,
}

impl<'a> From<::nom::Err<crate::syntax::Error<'a>>> for Error {
    fn from(err: ::nom::Err<crate::syntax::Error<'a>>) -> Self {
        let (message, labels) = match err {
            ::nom::Err::Incomplete(needed) => (
                format!(
                    "Need {} more token(s).",
                    match needed {
                        ::nom::Needed::Unknown => "a few".to_string(),
                        ::nom::Needed::Size(size) => size.to_string(),
                    }
                ),
                vec![Label {
                    location: Location::Eof,
                    message: None,
                }],
            ),
            ::nom::Err::Error(error) | ::nom::Err::Failure(error) => {
                let token = &error.input.tokens[0];
                (
                    format!("Unexpected token: {}", token.slice),
                    vec![Label {
                        location: Location::Known(token.span.clone()),
                        message: None,
                    }],
                )
            }
        };
        Error {
            code: ErrorCode::SyntaxError,
            message,
            labels,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
