use crate::{
    core::ast::{Expression, Literal},
    syntax::{parse::*, Token},
};

pub fn parse_literal_string(s: ParseInput) -> ParseResult<Literal> {
    let (s, token) = any(s)?;
    if let Token::LiteralString(v) = token {
        Ok((s, Literal::String(v.clone())))
    } else {
        err_tag(s)
    }
}
pub fn parse_literal_character(s: ParseInput) -> ParseResult<Literal> {
    let (s, token) = any(s)?;
    if let Token::LiteralCharacter(v) = token {
        Ok((s, Literal::Character(v.clone())))
    } else {
        err_tag(s)
    }
}

pub fn parse_literal_boolean(s: ParseInput) -> ParseResult<Literal> {
    let (s, token) = any(s)?;
    if let Token::LiteralBoolean(v) = token {
        Ok((s, Literal::Boolean(v.clone())))
    } else {
        err_tag(s)
    }
}

pub fn parse_literal_integer(s: ParseInput) -> ParseResult<Literal> {
    let (s, token) = any(s)?;
    if let Token::LiteralNumberIntegral(v) = token {
        Ok((s, Literal::Integer(v.clone())))
    } else {
        err_tag(s)
    }
}
pub fn parse_literal_decimal(s: ParseInput) -> ParseResult<Literal> {
    let (s, token) = any(s)?;
    if let Token::LiteralNumberDecimal(v) = token {
        Ok((s, Literal::Decimal(v.clone())))
    } else {
        err_tag(s)
    }
}

pub fn parse_literal(s: ParseInput) -> ParseResult<Literal> {
    alt((
        parse_literal_string,
        parse_literal_character,
        parse_literal_boolean,
        parse_literal_integer,
        parse_literal_decimal,
    ))(s)
}

pub fn parse_literal_expression(s: ParseInput) -> ParseResult<Expression> {
    map(parse_literal, Expression::Literal)(s)
}
