use crate::{
    core::ast::{Declaration, FunctionDeclaration, Type},
    syntax::{parse::*, Token},
};

pub fn parse_function_declaration(i: ParseInput) -> ParseResult<FunctionDeclaration> {
    let (i, extern_token) = opt(tag(Token::KeywordExtern))(i)?;
    let (i, _) = tag(Token::KeywordFn)(i)?;
    let (i, name) = parse_identifier_content(i)?;

    let (i, parameters) = opt(delimited(
        tag(Token::PunctuationLeftParenthesis),
        |i| {
            let (i, res) = separated_list0(tag(Token::PunctuationComma), |i| {
                let (i, pattern) = parse_pattern(i)?;
                let (i, ty) = parse_type_annotation(i)?;
                Ok((i, (pattern, ty)))
            })(i)?;
            if res.len() > 0 {
                let (i, _) = opt(tag(Token::PunctuationComma))(i)?;
                Ok((i, res))
            } else {
                Ok((i, res))
            }
        },
        tag(Token::PunctuationRightParenthesis),
    ))(i)?;

    let (i, return_type) = opt(preceded(tag(Token::PunctuationColon), parse_type))(i)?;
    let return_type = return_type.unwrap_or_else(|| Type::Tuple(Vec::new()));

    let (i, (body, last_expression)) = alt((
        map(tag(Token::PunctuationSemicolon), |_| (Vec::new(), None)),
        delimited(
            tag(Token::PunctuationLeftCurlyBracket),
            tuple((many0(parse_statement), opt(parse_expression))),
            tag(Token::PunctuationRightCurlyBracket),
        ),
    ))(i)?;

    Ok((
        i,
        FunctionDeclaration {
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
