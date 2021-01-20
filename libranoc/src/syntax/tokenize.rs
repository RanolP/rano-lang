use logos::Logos;

#[derive(Debug, Default)]
pub struct TokenExtras {
    line: usize,
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(extras = TokenExtras)]
pub enum Token {
    // #========== Punctuation ==========#
    #[token("!")]
    PunctuationExclamationMark,
    #[token("#")]
    PunctuationNumberSign,
    #[token("$")]
    PunctuationDollarSign,
    #[token("%")]
    PunctuationPercentSign,
    #[token("&")]
    PunctuationAmpersand,
    #[token("*")]
    PunctuationAsterisk,
    #[token("+")]
    PunctuationPlusSign,
    #[token(",")]
    PunctuationComma,
    #[token("-")]
    PunctuationHyphenMinus,
    #[token(".")]
    PunctuationFullStop,
    #[token("/")]
    PunctuationSolidus,
    #[token(":")]
    PunctuationColon,
    #[token(";")]
    PunctuationSemicolon,
    #[token("<")]
    PunctuationLessThanSign,
    #[token("=")]
    PunctuationEqualsSign,
    #[token(">")]
    PunctuationGreaterThanSign,
    #[token("?")]
    PunctuationQuestionMark,
    #[token("@")]
    PunctuationCommercialAt,
    #[token("\\")]
    PunctuationReverseSolidus,
    #[token("^")]
    PunctuationCircumflexAccent,
    #[token("|")]
    PunctuationVerticalLine,
    #[token("~")]
    PunctuationTilde,
    #[token("(")]
    PunctuationLeftParenthesis,
    #[token("[")]
    PunctuationLeftSquareBracket,
    #[token("{")]
    PunctuationLeftCurlyBracket,
    #[token(")")]
    PunctuationRightParenthesis,
    #[token("]")]
    PunctuationRightSquareBracket,
    #[token("}")]
    PunctuationRightCurlyBracket,
    // #========== Punctuations ==========#
    #[token("&&")]
    PunctuationsLogicalAnd,
    #[token("||")]
    PunctuationsLogicalOr,
    #[token("==")]
    PunctuationsEqualTo,
    #[token("!=")]
    PunctuationsNotEqualTo,
    #[token("<=")]
    PunctuationsLessThanOrEqualTo,
    #[token(">=")]
    PunctuationsGreaterThanOrEqualTo,
    #[token("->")]
    PunctuationsSingleRightArrow,
    #[token("..")]
    PunctuationsRangeRightExclusive,
    #[token("..=")]
    PunctuationsRangeRightInclusive,
    #[token("?.")]
    PunctuationsGetFieldNullable,
    // #========== Keyword ==========#
    #[token("as")]
    KeywordAs,
    #[token("break")]
    KeywordBreak,
    #[token("continue")]
    KeywordContinue,
    #[token("else")]
    KeywordElse,
    #[token("extern")]
    KeywordExtern,
    #[token("fn")]
    KeywordFn,
    #[token("for")]
    KeywordFor,
    #[token("If")]
    KeywordIf,
    #[token("impl")]
    KeywordImpl,
    #[token("in")]
    KeywordIn,
    #[token("let")]
    KeywordLet,
    #[token("match")]
    KeywordMatch,
    #[token("return")]
    KeywordReturn,
    #[token("self")]
    KeywordSelf,
    #[token("Self")]
    KeywordSelfType,
    #[token("struct")]
    KeywordStruct,
    #[token("trait")]
    KeywordTrait,
    #[token("type")]
    KeywordType,
    #[token("union")]
    KeywordUnion,
    #[token("use")]
    KeywordUse,
    #[token("where")]
    KeywordWhere,
    #[token("while")]
    KeywordWhile,

    // #========== Identifier ==========#
    #[regex(
        "[^0-9\n\u{000B}\u{000C}\r\u{0085}\u{2028}\u{2029}\t \u{00AD}\u{00A0}\u{1680}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{200A}\u{200B}\u{200E}\u{200F}\u{202F}\u{205F}\u{3000}\u{FEFF}!#$%&*+,-./:;<=>?@\\^|~(\\[{)\\]}][^\n\u{000B}\u{000C}\r\u{0085}\u{2028}\u{2029}\t \u{00AD}\u{00A0}\u{1680}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{200A}\u{200B}\u{200E}\u{200F}\u{202F}\u{205F}\u{3000}\u{FEFF}!#$%&*+,-./:;<=>?@\\^|~(\\[{)\\]}]*",
        callback = |lex| lex.slice().to_owned()
    )]
    IdentifierIdentifier(String),
    #[token("_")]
    KeywordPlaceholderName,

    // #========== Literal ==========#
    #[regex(r#"'(\\'|[^']*[^\\])'"#, |lex| lex.slice().to_owned())]
    LiteralCharacter(String),
    #[regex(r#"(""|"(\\"|[^"])*[^\\]")"#, |lex| lex.slice().to_owned())]
    LiteralString(String),
    #[regex("([0-9]+|0b[0-1]+|0o[0-7]+|0x[0-9a-fA-F]+)", |lex| lex.slice().to_owned())]
    LiteralNumberIntegral(String),
    #[regex("[0-9]+\\.[0-9]+", |lex| lex.slice().to_owned())]
    LiteralNumberDecimal(String),
    #[regex("[0-9]+(\\.[0-9]+)?[eE][+-][0-9]+", |lex| lex.slice().to_owned())]
    LiteralNumberExponent(String),
    #[regex("(true|false)", |lex| lex.slice().to_owned())]
    LiteralBoolean(String),

    /*
     * '\n'       : LINE FEED
     * '\u{000B}' : LINE TABULATION
     * '\u{000C}' : FORM FEED
     * '\r'       : CARRIAGE RETURN
     * '\u{0085}' : NEXT LINE
     * '\u{2028}' : LINE SEPARATOR
     * '\u{2029}' : PARAGRAPH SEPARATOR
     */
    #[regex(
        "(\r\n|[\n\u{000B}\u{000C}\r\u{0085}\u{2028}\u{2029}])",
        priority = 2,
        callback = |lex| {
            lex.extras.line += 1;

            logos::Skip
        }
    )]
    VerticalSpace,
    /*
     * '\t'       : CHARACTER TABULATION
     * ' '        : SPACE
     * '\u{00AD}' : SOFT HYPHEN
     * '\u{00A0}' : NO-BREAK SPACE
     * '\u{1680}' : OGHAM SPACE MARK
     * '\u{2000}' : EN QUAD
     * '\u{2001}' : EM QUAD
     * '\u{2002}' : EN SPACE
     * '\u{2003}' : EM SPACE
     * '\u{2004}' : THREE-PER-EM SPACE
     * '\u{2005}' : FOUR-PER-EM SPACE
     * '\u{2006}' : SIX-PER-EM SPACE
     * '\u{2007}' : FIGURE SPACE
     * '\u{2008}' : PUNCTUATION SPACE
     * '\u{2009}' : THIN SPACE
     * '\u{200A}' : HAIR SPACE
     * '\u{200B}' : ZERO WIDTH SPACE
     * '\u{200E}' : LEFT-TO-RIGHT MARK
     * '\u{200F}' : RIGHT-TO-LEFT MARK
     * '\u{202F}' : NARROW NO-BREAK SPACE
     * '\u{205F}' : MEDIUM MATHEMATICAL SPACE
     * '\u{3000}' : IDEPGRAPHIC SPACE
     * '\u{FEFF}' : ZERO WIDTH NO-BREAK SPACE
     */
    #[regex(
        "[\t \u{00AD}\u{00A0}\u{1680}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{200A}\u{200B}\u{200E}\u{200F}\u{202F}\u{205F}\u{3000}\u{FEFF}]+",
        logos::skip,
    )]
    HorizontalSpaces,
    #[error]
    Error,
}

pub fn create_tokenizer<'a>(src: &'a str) -> logos::Lexer<'a, Token> {
    Token::lexer(src)
}

pub fn tokenize(src: &str) -> Vec<Token> {
    create_tokenizer(src).collect()
}
