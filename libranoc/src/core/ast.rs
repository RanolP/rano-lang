use std::fmt;

use crate::syntax::Token;

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

#[derive(Debug, PartialEq)]
pub enum Statement {
    Declaration(Declaration),
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Declaration {
    FunctionDeclaration(FunctionDeclaration),
    VariableDeclaration,
    StructDeclaration,
    UnionDeclaration,
    TypeDeclaration,
    TraitDeclaration,
    ImplDeclaration,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub is_pub: bool,
    pub is_extern: bool,
    pub name: Token,
    // pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<(Pattern, Type)>,
    pub return_type: Type,
    // pub where_clauses: Vec<WhereClause>,
    pub body: Vec<Statement>,
    pub last_expression: Option<Expression>,
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
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Character(String),
    Integer(String),
    Decimal(String),
    Boolean(String),
}

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
    LogicalOr(LogicalOr),
    LogicalAnd(LogicalAnd),
    EqualTo(EqualTo),
    NotEqualTo(NotEqualTo),
    GreaterThan(GreaterThan),
    LessThan(LessThan),
    GreaterThanOrEqualTo(GreaterThanOrEqualTo),
    LessThanOrEqualTo(LessThanOrEqualTo),
    Add(Add),
    Subtract(Subtract),
    Multiply(Multiply),
    Divide(Divide),
    Remainder(Remainder),
    GetField(GetField),
    GetFieldNullable(GetFieldNullable),
    RangeRightExclusive(RangeRightExclusive),
    RangeRightInclusive(RangeRightInclusive),
}

#[derive(Debug, PartialEq, Clone)]
pub struct LogicalOr(pub Box<Expression>, pub Box<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub struct LogicalAnd(pub Box<Expression>, pub Box<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub struct EqualTo(pub Box<Expression>, pub Box<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub struct NotEqualTo(pub Box<Expression>, pub Box<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub struct GreaterThan(pub Box<Expression>, pub Box<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub struct LessThan(pub Box<Expression>, pub Box<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub struct GreaterThanOrEqualTo(pub Box<Expression>, pub Box<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub struct LessThanOrEqualTo(pub Box<Expression>, pub Box<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub struct Add(pub Box<Expression>, pub Box<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub struct Subtract(pub Box<Expression>, pub Box<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub struct Multiply(pub Box<Expression>, pub Box<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub struct Divide(pub Box<Expression>, pub Box<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub struct Remainder(pub Box<Expression>, pub Box<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub struct GetField(pub Box<Expression>, pub Box<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub struct GetFieldNullable(pub Box<Expression>, pub Box<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub struct RangeRightExclusive(pub Box<Expression>, pub Box<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub struct RangeRightInclusive(pub Box<Expression>, pub Box<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub enum PostfixOperator {
    Index(Index),
    FunctionCall(FunctionCall),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Index(pub Box<Expression>, pub Vec<Expression>);

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall(pub Box<Expression>, pub Vec<Expression>);

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

#[derive(Debug, PartialEq)]
pub enum Pattern {
    Slot(Name),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Name {
    Ident(Token),
    Placeholder,
}
