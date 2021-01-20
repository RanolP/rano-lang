use crate::{
    core::ast::{Expression, Literal},
    syntax::{parse::*, Token},
};

pub fn parse_literal_string(i: ParseInput) -> ParseResult<Literal> {
    let (i, token) = any(i)?;
    if let Token::LiteralString(v) = token {
        Ok((i, Literal::String(v.clone())))
    } else {
        err_tag(i)
    }
}
pub fn parse_literal_character(i: ParseInput) -> ParseResult<Literal> {
    let (i, token) = any(i)?;
    if let Token::LiteralCharacter(v) = token {
        Ok((i, Literal::Character(v.clone())))
    } else {
        err_tag(i)
    }
}

pub fn parse_literal_boolean(i: ParseInput) -> ParseResult<Literal> {
    let (i, token) = any(i)?;
    if let Token::LiteralBoolean(v) = token {
        Ok((i, Literal::Boolean(v.clone())))
    } else {
        err_tag(i)
    }
}

pub fn parse_literal_integer(i: ParseInput) -> ParseResult<Literal> {
    let (i, token) = any(i)?;
    if let Token::LiteralNumberIntegral(v) = token {
        Ok((i, Literal::Integer(v.clone())))
    } else {
        err_tag(i)
    }
}
pub fn parse_literal_decimal(i: ParseInput) -> ParseResult<Literal> {
    let (i, token) = any(i)?;
    if let Token::LiteralNumberDecimal(v) = token {
        Ok((i, Literal::Decimal(v.clone())))
    } else {
        err_tag(i)
    }
}

pub fn parse_literal(i: ParseInput) -> ParseResult<Literal> {
    alt((
        parse_literal_string,
        parse_literal_character,
        parse_literal_boolean,
        parse_literal_integer,
        parse_literal_decimal,
    ))(i)
}

pub fn parse_literal_expression(i: ParseInput) -> ParseResult<Expression> {
    map(parse_literal, Expression::Literal)(i)
}
