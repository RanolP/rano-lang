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
    // pub return_type: Type,
    // pub where_clauses: Vec<WhereClause>,
    pub body: Vec<Statement>,
    // pub last_expression: Option<Expression>,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Match,
    Closure,
    Literal(Literal),
    Path,
    Array,
    Tuple,
    Init,
    Name,
    Operator,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String,
    Integer,
    Decimal,
    Bool,
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
