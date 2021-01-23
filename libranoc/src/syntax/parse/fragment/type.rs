use crate::{core::ast::Type, syntax::parse::*};

pub fn parse_type_basic(i: ParseInput) -> ParseResult<Type> {
    let (i, base) = parse_path(i)?;

    // TODO: TypeParameters
    let type_parameters = Vec::new();

    Ok((
        i,
        Type::Basic {
            base,
            type_parameters,
        },
    ))
}

pub fn parse_type_impl(i: ParseInput) -> ParseResult<Type> {
    map(
        preceded(tag(TokenKind::KeywordImpl), cut(parse_type_basic)),
        |ty| Type::Impl(Box::new(ty)),
    )(i)
}

pub fn parse_type(i: ParseInput) -> ParseResult<Type> {
    alt((parse_type_basic, parse_type_impl))(i)
}

pub fn parse_type_annotation(i: ParseInput) -> ParseResult<Type> {
    preceded(tag(TokenKind::PunctuationColon), cut(parse_type))(i)
}
