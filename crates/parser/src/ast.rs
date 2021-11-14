pub type Identifier = String;
pub type Codeblock = Vec<Expression>;

#[derive(Debug)]
pub struct Type {
    name: Identifier
}

#[derive(Debug)]
pub struct Parameter {
    name: Identifier,
    type_: Type,
}

#[derive(Debug)]
pub struct Argument {
    expression: Expression
}

#[derive(Debug)]
pub enum Expression {
    FunctionDeclaration {
        name: Identifier,
        parameters: Vec<Parameter>,
        return_type: Option<Type>,
        body: Codeblock
    },
    FunctionCall {
        name: Identifier,
        arguments: Vec<Argument>
    },
    VariableDeclaration {
        name: Identifier,
        type_: Option<Type>,
        value: Option<Box<Expression>>
    },
    String(String),
    Integer(i64),
    VariableAccess(Identifier)
}