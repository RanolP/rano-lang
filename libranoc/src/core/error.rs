use thiserror::Error;

use crate::syntax::{Span, Token};

use super::ast::Type;

#[derive(Debug)]
#[repr(u16)]
pub enum ErrorCode {
    SyntaxError = 0001,
    Redefined = 0002,
    UndefinedSymbol = 0003,
    MismatchedType = 0004,
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

impl Error {
    pub fn redefined(name: String, before: Span, current: Span) -> Error {
        Error {
            code: ErrorCode::Redefined,
            message: format!("`{}` has been redefined.", name),
            labels: vec![
                Label {
                    location: Location::Known(before),
                    message: Some(format!("`{}` has been defined here...", name)),
                },
                Label {
                    location: Location::Known(current),
                    message: Some("But you have defined here too...".to_owned()),
                },
            ],
        }
    }
    pub fn undefined_symbol(name: Token) -> Error {
        Error {
            code: ErrorCode::UndefinedSymbol,
            message: format!("Undefined symbol `{}`.", name.content),
            labels: vec![Label {
                location: Location::Known(name.span.clone()),
                message: None,
            }],
        }
    }
    pub fn mismatched_type(required: Type, gotten: Type, location: Span) -> Error {
        Error {
            code: ErrorCode::MismatchedType,
            message: "Mismatched types.".to_string(),
            labels: vec![Label {
                location: Location::Known(location),
                message: Some(format!(
                    "Required `{}` but got `{}`",
                    required.to_string(),
                    gotten.to_string()
                )),
            }],
        }
    }
}

impl<'a> From<::nom::Err<crate::syntax::Error>> for Error {
    fn from(err: ::nom::Err<crate::syntax::Error>) -> Self {
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
                    format!("Unexpected token: {}", token.content),
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
