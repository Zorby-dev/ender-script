use std::num::TryFromIntError;
use std::ops::RangeBounds;

use parser::ast::{Expression, Type};
use utilities::cursor::Cursor;
use utilities::message::MessageType::*;
use utilities::message::{details, Message};

use crate::environment::{McFunction, Scope, Value};

/*fn ast_type_to_type(ast_type: &ast::Type) -> Result<Type, Message> {
    match ast_type.name.as_str() {
        "int" => Ok(Type::Int),
        _ => Err(Message::error(
            UnknownType,
            details::UnknownType!(ast_type.name),
            Cursor::new("fsdfdsf", "fsdfsdfsdfsdfsd"),
        )),
    }
}*/

struct Compiler {
    functions: Vec<McFunction>,
}
impl Compiler {
    fn new() -> Self {
        Self { functions: vec![] }
    }

    fn compile_variable_declaration(
        &self,
        cursor: Cursor,
        scope: &mut Scope,
        name: String,
        variable_type: Option<Type>,
        value: Option<Box<Expression>>,
    ) -> Result<Value, Message> {
        let player = format!("${}", name);
        let scoreboard = scope.function.name.clone();
        if scope.symbol_table.contains(&name) {
            return Err(Message::error(
                MemberRedeclaration,
                details::MemberRedeclaration!("Variable", name),
                cursor.clone()
            ));
        }
        scope.symbol_table.insert(name);
        if let Some(value) = value {
            let expr = self.compile_expression(scope, *value)?;
            scope.function.push_cmd(match expr {
                Value::Int(int) => {
                    format!("scoreboard players set {} {} {}", player, scoreboard, int)
                }
                Value::IntPointer {
                    scoreboard: other_scoreboard,
                    player: other_player,
                } => format!(
                    "scoreboard players operation {} {} = {} {}",
                    player, scoreboard, other_player, other_scoreboard
                ),
                _ => {
                    return Err(Message::error(
                        TypeMismatch,
                        details::TypeMismatch!("int", "unknown"),
                        cursor.clone(),
                    ))
                }
            });
            Ok(Value::IntPointer { scoreboard, player })
        } else {
            Ok(Value::UndefinedPointer { scoreboard, player })
        }
    }

    fn compile_integer(&self, cursor: Cursor, int: i64) -> Result<Value, Message> {
        let res: Result<i32, TryFromIntError> = int.try_into();
        match res {
            Ok(int) => Ok(Value::Int(int)),
            Err(_) => Err(Message::error(
                IntegerBoundsExceeded,
                details::IntegerBoundsExceeded!(32),
                cursor.clone(),
            )),
        }
    }

    fn compile_variable_access(
        &self,
        cursor: Cursor,
        scope: &Scope,
        identifier: String,
    ) -> Result<Value, Message> {
        if scope.symbol_table.contains(&identifier) {
            Ok(Value::IntPointer {
                scoreboard: scope.function.name.clone(),
                player: format!("${}", identifier),
            })
        } else {
            Err(Message::error(
                UnknownMember,
                details::UnknownMember!("Variable", identifier),
                cursor.clone()
            ))
        }
    }

    fn compile_expression(
        &self,
        scope: &mut Scope,
        expression: Expression,
    ) -> Result<Value, Message> {
        match expression {
            Expression::VariableDeclaration {
                name,
                variable_type,
                value,
                cursor,
            } => self.compile_variable_declaration(cursor, scope, name, variable_type, value),
            Expression::Integer(int, cursor) => self.compile_integer(cursor, int),
            Expression::VariableAccess(identifier, cursor) => {
                self.compile_variable_access(cursor, scope, identifier)
            }
            _ => unimplemented!(),
        }
    }

    fn compile_function_declaration(
        &mut self,
        name: String,
        body: Vec<Expression>,
    ) -> Result<Value, Message> {
        let mut function = McFunction::new(name.clone());
        function.push_cmd(format!("scoreboard objectives add {} dummy", name));
        let mut scope = Scope::new(&mut function);

        for expression in body {
            self.compile_expression(&mut scope, expression)?;
        }

        function.push_cmd(format!("scoreboard objectives remove {}", name));

        self.functions.push(function);
        Ok(Value::FunctionPointer(name))
    }
}

pub fn compile(ast: Vec<Expression>) -> Result<String, Message> {
    let mut compiler = Compiler::new();
    compiler.compile_function_declaration("main".to_string(), ast)?;
    Ok(compiler.functions.last().unwrap().to_string())
}
