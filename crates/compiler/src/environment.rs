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

pub enum Value {
    Int(i32),
    IntPointer { scoreboard: String, player: String },
    UndefinedPointer { scoreboard: String, player: String },
    FunctionPointer(String),
}

pub struct Scope<'a> {
    pub function: &'a mut McFunction,
    pub symbol_table: HashSet<String>
}
impl<'a> Scope<'a> {
    pub fn new(function: &'a mut McFunction) -> Self {
        Self {
            function,
            symbol_table: HashSet::new()
        }
    }
}