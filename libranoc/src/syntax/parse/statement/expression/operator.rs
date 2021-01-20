use crate::{
    core::ast::{Expression, InfixOperatorKind, PostfixOperatorKind, PrefixOperatorKind},
    syntax::{parse::*, Token},
};

pub enum AfterLhsOperator {
    Infix(InfixOperatorKind, Box<Expression>),
    Postfix(PostfixOperatorKind, Option<Box<Expression>>),
}

pub struct OperatorBindingPowerInfix {
    pub kind: InfixOperatorKind,
    pub left_binding_power: u8,
    pub right_binding_power: u8,
}

pub struct OperatorBindingPowerPrefix {
    pub kind: PrefixOperatorKind,
    pub right_binding_power: u8,
}

pub struct OperatorBindingPowerPostfix {
    pub kind: PostfixOperatorKind,
    pub left_binding_power: u8,
}

pub fn parse_prefix_operator(i: ParseInput) -> ParseResult<OperatorBindingPowerPrefix> {
    alt((
        map(tag(Token::PunctuationExclamationMark), |_| {
            OperatorBindingPowerPrefix {
                kind: PrefixOperatorKind::Not,
                right_binding_power: 13,
            }
        }),
        map(tag(Token::PunctuationPlusSign), |_| {
            OperatorBindingPowerPrefix {
                kind: PrefixOperatorKind::UnaryPlus,
                right_binding_power: 13,
            }
        }),
        map(tag(Token::PunctuationHyphenMinus), |_| {
            OperatorBindingPowerPrefix {
                kind: PrefixOperatorKind::UnaryMinus,
                right_binding_power: 13,
            }
        }),
    ))(i)
}

pub fn parse_infix_operator(i: ParseInput) -> ParseResult<OperatorBindingPowerInfix> {
    alt((
        map(tag(Token::PunctuationsLogicalOr), |_| {
            OperatorBindingPowerInfix {
                kind: InfixOperatorKind::LogicalOr,
                left_binding_power: 1,
                right_binding_power: 2,
            }
        }),
        map(tag(Token::PunctuationsLogicalAnd), |_| {
            OperatorBindingPowerInfix {
                kind: InfixOperatorKind::LogicalAnd,
                left_binding_power: 3,
                right_binding_power: 4,
            }
        }),
        map(tag(Token::PunctuationsEqualTo), |_| {
            OperatorBindingPowerInfix {
                kind: InfixOperatorKind::EqualTo,
                left_binding_power: 5,
                right_binding_power: 6,
            }
        }),
        map(tag(Token::PunctuationsNotEqualTo), |_| {
            OperatorBindingPowerInfix {
                kind: InfixOperatorKind::NotEqualTo,
                left_binding_power: 5,
                right_binding_power: 6,
            }
        }),
        map(tag(Token::PunctuationGreaterThanSign), |_| {
            OperatorBindingPowerInfix {
                kind: InfixOperatorKind::GreaterThan,
                left_binding_power: 7,
                right_binding_power: 8,
            }
        }),
        map(tag(Token::PunctuationLessThanSign), |_| {
            OperatorBindingPowerInfix {
                kind: InfixOperatorKind::LessThan,
                left_binding_power: 7,
                right_binding_power: 8,
            }
        }),
        map(tag(Token::PunctuationsGreaterThanOrEqualTo), |_| {
            OperatorBindingPowerInfix {
                kind: InfixOperatorKind::GreaterThanOrEqualTo,
                left_binding_power: 7,
                right_binding_power: 8,
            }
        }),
        map(tag(Token::PunctuationsLessThanOrEqualTo), |_| {
            OperatorBindingPowerInfix {
                kind: InfixOperatorKind::LessThanOrEqualTo,
                left_binding_power: 7,
                right_binding_power: 8,
            }
        }),
        map(tag(Token::PunctuationPlusSign), |_| {
            OperatorBindingPowerInfix {
                kind: InfixOperatorKind::Add,
                left_binding_power: 9,
                right_binding_power: 10,
            }
        }),
        map(tag(Token::PunctuationHyphenMinus), |_| {
            OperatorBindingPowerInfix {
                kind: InfixOperatorKind::Subtract,
                left_binding_power: 9,
                right_binding_power: 10,
            }
        }),
        map(tag(Token::PunctuationAsterisk), |_| {
            OperatorBindingPowerInfix {
                kind: InfixOperatorKind::Multiply,
                left_binding_power: 11,
                right_binding_power: 12,
            }
        }),
        map(tag(Token::PunctuationSolidus), |_| {
            OperatorBindingPowerInfix {
                kind: InfixOperatorKind::Divide,
                left_binding_power: 11,
                right_binding_power: 12,
            }
        }),
        map(tag(Token::PunctuationPercentSign), |_| {
            OperatorBindingPowerInfix {
                kind: InfixOperatorKind::Remainder,
                left_binding_power: 11,
                right_binding_power: 12,
            }
        }),
        map(tag(Token::PunctuationFullStop), |_| {
            OperatorBindingPowerInfix {
                kind: InfixOperatorKind::GetField,
                left_binding_power: 16,
                right_binding_power: 15,
            }
        }),
        map(tag(Token::PunctuationsGetFieldNullable), |_| {
            OperatorBindingPowerInfix {
                kind: InfixOperatorKind::GetFieldNullable,
                left_binding_power: 16,
                right_binding_power: 15,
            }
        }),
        map(tag(Token::PunctuationsRangeRightExclusive), |_| {
            OperatorBindingPowerInfix {
                kind: InfixOperatorKind::RangeRightExclusive,
                left_binding_power: 18,
                right_binding_power: 17,
            }
        }),
        map(tag(Token::PunctuationsRangeRightInclusive), |_| {
            OperatorBindingPowerInfix {
                kind: InfixOperatorKind::RangeRightInclusive,
                left_binding_power: 18,
                right_binding_power: 17,
            }
        }),
    ))(i)
}

pub fn parse_postfix_operator(i: ParseInput) -> ParseResult<OperatorBindingPowerPostfix> {
    map(tag(Token::PunctuationLeftSquareBracket), |_| {
        OperatorBindingPowerPostfix {
            kind: PostfixOperatorKind::Index,
            left_binding_power: 14,
        }
    })(i)
}
