use crate::syntax::Token;

#[derive(Debug, PartialEq)]
pub struct Module<'a> {
    pub(crate) nodes: Vec<Node<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum Node<'a> {
    Directive,
    Statement(Statement<'a>),
}

// TODO: directives like #![deny(unused_variable)]
#[derive(Debug, PartialEq)]
pub struct Directive {}

#[derive(Debug, PartialEq)]
pub enum Statement<'a> {
    Declaration(Declaration<'a>),
    Expression(Expression<'a>),
}

#[derive(Debug, PartialEq)]
pub enum Declaration<'a> {
    FunctionDeclaration(FunctionDeclaration<'a>),
    VariableDeclaration,
    StructDeclaration,
    UnionDeclaration,
    TypeDeclaration,
    TraitDeclaration,
    ImplDeclaration,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration<'a> {
    pub is_pub: bool,
    pub is_extern: bool,
    pub name: Identifier<'a>,
    // pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<(Pattern<'a>, Type<'a>)>,
    pub return_type: Type<'a>,
    // pub where_clauses: Vec<WhereClause>,
    pub body: Vec<Statement<'a>>,
    pub last_expression: Option<Expression<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression<'a> {
    Match,
    Closure,
    Literal(Literal),
    Path,
    Array,
    Tuple(Vec<Expression<'a>>),
    Init,
    Operator(Operator<'a>),
    Name(Name<'a>),
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
pub enum Operator<'a> {
    Prefix(PrefixOperatorKind, Box<Expression<'a>>),
    Infix(Box<Expression<'a>>, InfixOperatorKind, Box<Expression<'a>>),
    Postfix(
        Box<Expression<'a>>,
        PostfixOperatorKind,
        Vec<Expression<'a>>,
    ),
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrefixOperatorKind {
    Not,
    UnaryPlus,
    UnaryMinus,
}
#[derive(Debug, PartialEq, Clone)]
pub enum InfixOperatorKind {
    LogicalOr,
    LogicalAnd,
    EqualTo,
    NotEqualTo,
    GreaterThan,
    LessThan,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    GetField,
    GetFieldNullable,
    RangeRightExclusive,
    RangeRightInclusive,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PostfixOperatorKind {
    Index,
    FunctionCall,
}
#[derive(Debug, PartialEq)]
pub struct Path<'a>(pub Vec<Identifier<'a>>);

#[derive(Debug, PartialEq)]
pub enum TypeParameter<'a> {
    Star,
    Specific(Type<'a>),
}

#[derive(Debug, PartialEq)]
pub enum Type<'a> {
    Basic {
        base: Path<'a>,
        type_parameters: Vec<TypeParameter<'a>>,
    },
    Tuple(Vec<Type<'a>>),
    Impl(Box<Type<'a>>),
    Nullable(Box<Type<'a>>),
    Function {
        // generic and where clause?
        parameters_type: Vec<Type<'a>>,
        return_type: Box<Type<'a>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Pattern<'a> {
    Slot(Name<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Name<'a> {
    Ident(Identifier<'a>),
    Placeholder,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier<'a>(pub Token<'a>, pub String);
