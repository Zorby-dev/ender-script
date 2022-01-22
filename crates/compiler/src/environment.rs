use std::collections::HashSet;

pub struct McFunction {
    text: String,
    pub name: String,
}
impl McFunction {
    pub fn new(name: impl ToString) -> Self {
        Self {
            text: String::new(),
            name: name.to_string(),
        }
    }

    pub fn push_cmd(&mut self, cmd: impl ToString) {
        self.text += &(cmd.to_string() + "\n");
    }
}
impl ToString for McFunction {
    fn to_string(&self) -> String {
        self.text.clone()
    }
}

#[derive(Clone, PartialEq)]
pub enum Value {
    Int(i32),
    IntReference { scoreboard: String, player: String },
    Undefined,
    UndefinedReference { scoreboard: String, player: String },
    FunctionReference(String),
}

impl Value {
    pub fn name(&self) -> &'static str {
        match self {
            | Value::Int(_) | Value::IntReference { .. } => "int",
            | Value::Undefined | Value::UndefinedReference { .. } => "undefined",
            | Value::FunctionReference(_) => "function",
        }
    }
}

pub struct Scope<'a> {
    pub function: &'a mut McFunction,
    pub symbol_table: HashSet<String>,
    pub parent: Option<&'a Scope<'a>>,
}
impl<'a> Scope<'a> {
    pub fn new(function: &'a mut McFunction, parent: Option<&'a Scope<'a>>) -> Self {
        Self {
            function,
            symbol_table: HashSet::new(),
            parent,
        }
    }
}

#[derive(Clone)]
pub struct Context {
    pub macro_target: Option<Value>,
}
