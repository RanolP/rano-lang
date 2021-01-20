use crate::{core::ast::Type, syntax::parse::*};

pub fn parse_type_basic(s: ParseInput) -> ParseResult<Type> {
    let (s, base) = parse_path(s)?;

    // TODO: TypeParameters
    let type_parameters = Vec::new();

    Ok((
        s,
        Type::Basic {
            base,
            type_parameters,
        },
    ))
}

pub fn parse_type_impl(s: ParseInput) -> ParseResult<Type> {
    let (s, _) = tag(Token::KeywordImpl)(s)?;
    let (s, ty) = parse_type_basic(s)?;
    Ok((s, Type::Impl(Box::new(ty))))
}

pub fn parse_type(s: ParseInput) -> ParseResult<Type> {
    alt((parse_type_basic, parse_type_impl))(s)
}

pub fn parse_type_annotation(s: ParseInput) -> ParseResult<Type> {
    preceded(tag(Token::PunctuationColon), parse_type)(s)
}
