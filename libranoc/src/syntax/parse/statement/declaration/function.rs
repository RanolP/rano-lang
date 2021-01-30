use crate::{
    core::ast::{Declaration, FunctionDeclaration, Type},
    syntax::parse::*,
};

pub fn parse_function_declaration(i: ParseInput) -> ParseResult<FunctionDeclaration> {
    let (i, pub_token) = opt(tag(TokenKind::KeywordPub))(i)?;
    let (i, extern_token) = opt(tag(TokenKind::KeywordExtern))(i)?;
    let (i, _) = tag(TokenKind::KeywordFn)(i)?;
    let (i, name) = cut(parse_identifier)(i)?;

    let (i, parameters) = opt(delimited(
        tag(TokenKind::PunctuationLeftParenthesis),
        |i| {
            let (i, res) = separated_list0(tag(TokenKind::PunctuationComma), |i| {
                let (i, pattern) = parse_pattern(i)?;
                let (i, ty) = parse_type_annotation(i)?;
                Ok((i, (pattern, ty)))
            })(i)?;
            if res.len() > 0 {
                let (i, _) = opt(tag(TokenKind::PunctuationComma))(i)?;
                Ok((i, res))
            } else {
                Ok((i, res))
            }
        },
        tag(TokenKind::PunctuationRightParenthesis),
    ))(i)?;

    let (i, return_type) = opt(preceded(tag(TokenKind::PunctuationColon), parse_type))(i)?;
    let return_type = return_type.unwrap_or_else(|| Type::Tuple(Vec::new()));

    let (i, (body, last_expression)) = cut(alt((
        map(tag(TokenKind::PunctuationSemicolon), |_| (Vec::new(), None)),
        delimited(
            tag(TokenKind::PunctuationLeftCurlyBracket),
            tuple((many0(parse_statement), opt(parse_expression))),
            tag(TokenKind::PunctuationRightCurlyBracket),
        ),
    )))(i)?;

    Ok((
        i,
        FunctionDeclaration {
            is_pub: pub_token.is_some(),
            is_extern: extern_token.is_some(),
            name,
            parameters: parameters.unwrap_or_else(|| Vec::new()),
            return_type,
            body,
            last_expression,
        },
    ))
}

pub fn parse_function_declaration_declaration(i: ParseInput) -> ParseResult<Declaration> {
    let (s, declaration) = parse_function_declaration(i)?;
    Ok((s, Declaration::FunctionDeclaration(declaration)))
}
