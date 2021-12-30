use utilities::cursor::Cursor;

pub type Identifier = String;
pub type Codeblock = Vec<Expression>;

#[derive(Debug)]
pub struct Type {
    pub name: Identifier,
}

#[derive(Debug)]
pub struct Parameter {
    pub name: Identifier,
    pub type_: Type,
}

#[derive(Debug)]
pub struct Argument {
    expression: Expression,
}

#[derive(Debug)]
pub enum Expression {
    FunctionDeclaration {
        name: Option<Identifier>,
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
    RawCode {
        string: String,
        cursor: Cursor
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
    String(String, Cursor),
    Integer(i64, Cursor),
    VariableAccess(Identifier, Cursor),
}
