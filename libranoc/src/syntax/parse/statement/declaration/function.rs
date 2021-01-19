use crate::{
    core::ast::{Declaration, FunctionDeclaration},
    syntax::{parse::*, Token},
};

pub fn parse_function_declaration(s: ParseInput) -> ParseResult<FunctionDeclaration> {
    let (s, extern_token) = opt(tag(Token::KeywordExtern))(s)?;
    let (s, _) = tag(Token::KeywordFn)(s)?;
    let (s, name) = parse_identifier_content(s)?;

    let (s, parameters) = opt(delimited(
        tag(Token::PunctuationLeftParenthesis),
        |s| {
            let (s, res) = separated_list0(tag(Token::PunctuationComma), |s| {
                let (s, pattern) = parse_pattern(s)?;
                let (s, ty) = parse_type_annotation(s)?;
                Ok((s, (pattern, ty)))
            })(s)?;
            if res.len() > 0 {
                let (s, _) = opt(tag(Token::PunctuationComma))(s)?;
                Ok((s, res))
            } else {
                Ok((s, res))
            }
        },
        tag(Token::PunctuationRightParenthesis),
    ))(s)?;

    Ok((
        s,
        FunctionDeclaration {
            is_extern: extern_token.is_some(),
            name,
            parameters: parameters.unwrap_or_else(|| Vec::new()),
            body: vec![],
        },
    ))
}

pub fn parse_function_declaration_declaration(s: ParseInput) -> ParseResult<Declaration> {
    let (s, declaration) = parse_function_declaration(s)?;
    Ok((s, Declaration::FunctionDeclaration(declaration)))
}
