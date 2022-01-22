use utilities::cursor::Cursor;

pub type Identifier = String;
pub type Codeblock = Vec<Expression>;

#[derive(Debug, Clone)]
pub struct Type {
    pub name: Identifier,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: Identifier,
    pub type_: Type,
}

#[derive(Debug, Clone)]
pub struct Argument {
    expression: Expression,
}

#[derive(Debug, Clone)]
pub enum Expression {
    FunctionDeclaration {
        name: Identifier,
        parameters: Vec<Parameter>,
        return_type: Option<Type>,
        body: Codeblock,
        cursor: Cursor,
    },
    FunctionCall {
        name: Identifier,
        arguments: Vec<Argument>,
        cursor: Cursor,
    },
    VariableDeclaration {
        name: Identifier,
        variable_type: Option<Type>,
        value: Option<Box<Expression>>,
        cursor: Cursor,
    },
    VariableAssign {
        name: Identifier,
        value: Box<Expression>,
        cursor: Cursor,
    },
    RawCode {
        string: String,
        cursor: Cursor,
    },
    Addition {
        left: Box<Expression>,
        right: Box<Expression>,
        cursor: Cursor,
    },
    Subtraction {
        left: Box<Expression>,
        right: Box<Expression>,
        cursor: Cursor,
    },
    Multiplication {
        left: Box<Expression>,
        right: Box<Expression>,
        cursor: Cursor,
    },
    Division {
        left: Box<Expression>,
        right: Box<Expression>,
        cursor: Cursor,
    },
    String(
        String,
        Cursor,
    ),
    Integer(
        i64,
        Cursor,
    ),
    VariableAccess(
        Identifier,
        Cursor,
    ),
}

impl Expression {
    pub fn get_cursor(&self) -> &Cursor {
        match self {
            | Expression::FunctionDeclaration { cursor, .. } => cursor,
            | Expression::FunctionCall { cursor, .. } => cursor,
            | Expression::VariableDeclaration { cursor, .. } => cursor,
            | Expression::VariableAssign { cursor, .. } => cursor,
            | Expression::RawCode { cursor, .. } => cursor,
            | Expression::Addition { cursor, .. } => cursor,
            | Expression::Subtraction { cursor, .. } => cursor,
            | Expression::Multiplication { cursor, .. } => cursor,
            | Expression::Division { cursor, .. } => cursor,
            | Expression::String(_, cursor) => cursor,
            | Expression::Integer(_, cursor) => cursor,
            | Expression::VariableAccess(_, cursor) => cursor,
        }
    }
}
