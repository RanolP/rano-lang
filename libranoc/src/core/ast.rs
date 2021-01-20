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
    pub is_extern: bool,
    pub name: String,
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
    Name,
    Operator(Operator),
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
    Prefix(PrefixOperatorKind, Box<Expression>),
    Infix(Box<Expression>, InfixOperatorKind, Box<Expression>),
    Postfix(
        Box<Expression>,
        PostfixOperatorKind,
        Option<Box<Expression>>,
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
}
#[derive(Debug, PartialEq)]
pub struct Path(pub Vec<String>);

#[derive(Debug, PartialEq)]
pub enum TypeParameter {
    Star,
    Specific(Type),
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Pattern {
    Slot(Name),
}

#[derive(Debug, PartialEq)]
pub enum Name {
    Ident(String),
    Placeholder,
}
