use std::fmt;

use crate::syntax::{Span, Spanned, Token};

#[derive(Debug, PartialEq)]
pub struct Module {
    pub(crate) nodes: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Directive,
    Statement(Statement),
}

// TODO: directives like #![deny(unused_variable)]
#[derive(Debug, PartialEq)]
pub struct Directive {}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Declaration(Declaration),
    Expression(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    FunctionDeclaration(FunctionDeclaration),
    VariableDeclaration,
    StructDeclaration,
    UnionDeclaration,
    TypeDeclaration,
    TraitDeclaration,
    ImplDeclaration,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    pub is_pub: bool,
    pub is_extern: bool,
    pub name: Token,
    // pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<(Pattern, Type)>,
    pub return_type: Type,
    // pub where_clauses: Vec<WhereClause>,
    pub body: Option<Block>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Match,
    Closure,
    Literal(Literal),
    Path,
    Array,
    Tuple(Vec<Expression>),
    Init,
    Operator(Operator),
    Name(Name),
    If(If),
}

impl Spanned for Expression {
    fn span(&self) -> crate::syntax::Span {
        match self {
            _ => todo!("SPAN PLEASE"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Character(String),
    Integer(Integer),
    Decimal(String),
    Boolean(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Integer(pub String);

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Prefix(PrefixOperator),
    Infix(InfixOperator),
    Postfix(PostfixOperator),
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrefixOperator {
    Not(Not),
    UnaryPlus(UnaryPlus),
    UnaryMinus(UnaryMinus),
}
#[derive(Debug, PartialEq, Clone)]
pub struct Not(pub Box<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryPlus(pub Box<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryMinus(pub Box<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub enum InfixOperator {
    LogicalOr(Box<Expression>, Span, Box<Expression>),
    LogicalAnd(Box<Expression>, Span, Box<Expression>),
    EqualTo(Box<Expression>, Span, Box<Expression>),
    NotEqualTo(Box<Expression>, Span, Box<Expression>),
    GreaterThan(Box<Expression>, Span, Box<Expression>),
    LessThan(Box<Expression>, Span, Box<Expression>),
    GreaterThanOrEqualTo(Box<Expression>, Span, Box<Expression>),
    LessThanOrEqualTo(Box<Expression>, Span, Box<Expression>),
    Add(Box<Expression>, Span, Box<Expression>),
    Subtract(Box<Expression>, Span, Box<Expression>),
    Multiply(Box<Expression>, Span, Box<Expression>),
    Divide(Box<Expression>, Span, Box<Expression>),
    Remainder(Box<Expression>, Span, Box<Expression>),
    GetField(GetField),
    GetFieldNullable(GetFieldNullable),
    RangeRightExclusive(Box<Expression>, Span, Box<Expression>),
    RangeRightInclusive(Box<Expression>, Span, Box<Expression>),
}

impl InfixOperator {
    pub fn trait_name(&self) -> &'static str {
        match self {
            InfixOperator::LogicalOr(..) => "LogicalOr",
            InfixOperator::LogicalAnd(..) => "LogicalAnd",
            InfixOperator::EqualTo(..) => "PartialEq",
            InfixOperator::NotEqualTo(..) => "PartialEq",
            InfixOperator::GreaterThan(..) => "PartialOrd",
            InfixOperator::LessThan(..) => "PartialOrd",
            InfixOperator::GreaterThanOrEqualTo(..) => "PartialOrd",
            InfixOperator::LessThanOrEqualTo(..) => "PartialOrd",
            InfixOperator::Add(..) => "Add",
            InfixOperator::Subtract(..) => "Subtract",
            InfixOperator::Multiply(..) => "Multiply",
            InfixOperator::Divide(..) => "Divide",
            InfixOperator::Remainder(..) => "Remainder",
            InfixOperator::GetField(..) => "GetField",
            InfixOperator::GetFieldNullable(..) => "GetField",
            InfixOperator::RangeRightExclusive(..) => "RangeToExlusive",
            InfixOperator::RangeRightInclusive(..) => "RangeToInclusive",
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GetField(pub Box<Expression>, pub Box<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub struct GetFieldNullable(pub Box<Expression>, pub Box<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub enum PostfixOperator {
    Index(Index),
    FunctionCall(FunctionCall),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Index(pub Box<Expression>, pub Vec<Expression>);

impl Spanned for Index {
    fn span(&self) -> crate::syntax::Span {
        self.0.span().joined(&self.1.span())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall(pub Box<Expression>, pub Vec<Expression>);

impl Spanned for FunctionCall {
    fn span(&self) -> crate::syntax::Span {
        self.0.span().joined(&self.1.span())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Path(pub Vec<Token>);

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|token| token.content.clone())
                .collect::<Vec<_>>()
                .join(".")
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeParameter {
    Star,
    Specific(Type),
}

impl fmt::Display for TypeParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeParameter::Star => {
                write!(f, "*")
            }
            TypeParameter::Specific(ty) => {
                write!(f, "{}", ty)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Basic {
        base: Path,
        type_parameters: Vec<TypeParameter>,
    },
    Tuple(Vec<Type>),
    Impl(Box<Type>),
    Nullable(Box<Type>),
    Function {
        // generic and where clause?
        parameters_type: Vec<Type>,
        return_type: Box<Type>,
    },
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Basic {
                base,
                type_parameters,
            } => {
                write!(
                    f,
                    "{}<{}>",
                    base,
                    type_parameters
                        .iter()
                        .map(|type_parameter| type_parameter.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Type::Tuple(types) => {
                write!(
                    f,
                    "({})",
                    types
                        .iter()
                        .map(|ty| format!("{},", ty))
                        .collect::<Vec<_>>()
                        .join("")
                )
            }
            Type::Impl(ty) => {
                write!(f, "impl {}", ty)
            }
            Type::Nullable(ty) => {
                write!(f, "{}?", ty)
            }
            Type::Function {
                parameters_type,
                return_type,
            } => {
                write!(
                    f,
                    "({}) -> {}",
                    parameters_type
                        .iter()
                        .map(|ty| ty.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    return_type
                )
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    Slot(Name),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Name {
    Ident(Token),
    Placeholder,
}

#[derive(Debug, PartialEq, Clone)]
pub struct If {
    pub if_token: Token,
    pub condition: Box<Expression>,
    pub body: Box<Block>,
    pub else_part: Option<Else>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Else {
    If(Token, Box<If>),
    Block(Token, Box<Block>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub curly_bracket_open_token: Token,
    pub body: Vec<Statement>,
    pub last_expression: Option<Expression>,
    pub curly_bracket_close_token: Token,
}
