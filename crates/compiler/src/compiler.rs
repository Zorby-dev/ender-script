use std::num::TryFromIntError;

use parser::ast::{Expression, Type};
use utilities::cursor::Cursor;
use utilities::message::MessageType::*;
use utilities::message::{details, Message};

use crate::environment::{McFunction, Scope, Value, Context};

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

enum MathOp {
    Addition,
    Subtraction,
    Multiplication,
    Division
}
impl MathOp {
    fn symbol(&self) -> &'static char {
        match self {
            MathOp::Addition => &'+',
            MathOp::Subtraction => &'-',
            MathOp::Multiplication => &'*',
            MathOp::Division => &'/',
        }
    }

    fn execute(&self, left: i32, right: i32) -> i32 {
        match self {
            MathOp::Addition => left + right,
            MathOp::Subtraction => left - right,
            MathOp::Multiplication => left * right,
            MathOp::Division => left / right,
        }
    }
}

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
        context: &Context,
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
                cursor.clone(),
            ));
        }
        if let Some(value) = value {
            let reference = Value::IntReference {
                scoreboard: scoreboard.clone(),
                player: player.clone(),
            };
            let mut new_context = context.clone();
            new_context.macro_target = Some(reference.clone());
            let expr = self.compile_expression(scope, &new_context, *value)?;
            if expr == reference {
                return Ok(reference);
            }
            scope.function.push_cmd(match expr {
                Value::Int(int) => {
                    format!("scoreboard players set {} {} {}", &player, &scoreboard, int)
                }
                Value::IntReference {
                    scoreboard: other_scoreboard,
                    player: other_player,
                } => format!(
                    "scoreboard players operation {} {} = {} {}",
                    &player, &scoreboard, other_player, other_scoreboard
                ),
                _ => {
                    return Err(Message::error(
                        TypeMismatch,
                        details::TypeMismatch!("int", "unknown"),
                        cursor.clone(),
                    ))
                }
            });
            scope.symbol_table.insert(name);
            Ok(reference)
        } else {
            scope.symbol_table.insert(name);
            Ok(Value::UndefinedReference { scoreboard, player })
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
            Ok(Value::IntReference {
                scoreboard: scope.function.name.clone(),
                player: format!("${}", identifier),
            })
        } else {
            Err(Message::error(
                UnknownMember,
                details::UnknownMember!("Variable", identifier),
                cursor.clone(),
            ))
        }
    }

    fn execute_semi_static_math_operation(&self, scope: &mut Scope, math_op: MathOp, player: &String, scoreboard: &String, value: i32) {
        match math_op {
            MathOp::Addition => {
                scope.function.push_cmd(format!(
                    "scoreboard players add {} {} {}",
                    player, scoreboard, value
                ));
            },
            MathOp::Subtraction => {
                scope.function.push_cmd(format!(
                    "scoreboard players remove {} {} {}",
                    player, scoreboard, value
                ));
            },
            MathOp::Multiplication => {
                scope.function.push_cmd(format!(
                    "scoreboard players set %{} {} {}",
                    value, scope.function.name, value
                ));
                scope.function.push_cmd(format!(
                    "scoreboard players operation {} {} *= %{} {}",
                    player, scoreboard, value, scope.function.name
                ));
            },
            MathOp::Division => {
                scope.function.push_cmd(format!(
                    "scoreboard players set %{} {} {}",
                    value, scope.function.name, value
                ));
                scope.function.push_cmd(format!(
                    "scoreboard players operation {} {} /= %{} {}",
                    player, scoreboard, value, scope.function.name
                ));
            },
        }
    }

    fn compile_math_operation(
        &self,
        cursor: Cursor,
        scope: &mut Scope,
        context: &Context,
        math_op: MathOp,
        left_expr: Expression,
        right_expr: Expression,
    ) -> Result<Value, Message> {
        let left = self.compile_expression(scope, context, left_expr)?;
        let mut new_context = context.clone();
        new_context.macro_target = None;
        let right = self.compile_expression(scope, &new_context, right_expr)?;

        match (left, right) {
            (Value::Int(left_val), Value::Int(right_val)) => Ok(Value::Int(math_op.execute(left_val, right_val))),

            (Value::IntReference { player, scoreboard }, Value::Int(right_val)) => {
                let (temp, temp_scoreboard) = match &context.macro_target {
                    Some(_out @ Value::IntReference { player, scoreboard }) => {
                        (player.to_string(), scoreboard.to_string())
                    }
                    None => ("$$temp".to_string(), scope.function.name.clone()),
                    _ => unreachable!(),
                };

                if (&player, &scoreboard) != (&temp, &temp_scoreboard) {
                    scope.function.push_cmd(format!(
                        "scoreboard players operation {} {} = {} {}",
                        temp, temp_scoreboard, &player, &scoreboard
                    ));
                }

                self.execute_semi_static_math_operation(scope, math_op, &temp, &temp_scoreboard, right_val);

                Ok(Value::IntReference {
                    player: temp.to_string(),
                    scoreboard: scope.function.name.to_string(),
                })
            }
            (Value::Int(left_val), Value::IntReference { player, scoreboard }) => {
                let (temp, temp_scoreboard) = match &context.macro_target {
                    Some(_out @ Value::IntReference { player, scoreboard }) => {
                        (player.to_string(), scoreboard.to_string())
                    }
                    None => ("$$temp".to_string(), scope.function.name.clone()),
                    _ => unreachable!(),
                };

                scope.function.push_cmd(format!(
                    "scoreboard players set {} {} {}",
                    temp, temp_scoreboard, left_val
                ));

                scope.function.push_cmd(format!(
                    "scoreboard players operation {} {} {}= {} {}",
                    temp, temp_scoreboard, math_op.symbol(), &player, &scoreboard
                ));

                Ok(Value::IntReference {
                    player: temp.to_string(),
                    scoreboard: scope.function.name.to_string(),
                })
            }
            (
                Value::IntReference {
                    player: left_player,
                    scoreboard: left_scoreboard,
                },
                Value::IntReference {
                    player: right_player,
                    scoreboard: right_scoreboard,
                },
            ) => {
                let (temp, temp_scoreboard) = match &context.macro_target {
                    Some(_out @ Value::IntReference { player, scoreboard }) => {
                        (player.to_string(), scoreboard.to_string())
                    }
                    None => ("$$temp".to_string(), scope.function.name.clone()),
                    _ => unreachable!(),
                };

                if (&left_player, &left_scoreboard) != (&temp, &temp_scoreboard) {
                    scope.function.push_cmd(format!(
                        "scoreboard players operation {} {} = {} {}",
                        temp, temp_scoreboard, &left_player, &left_scoreboard
                    ));
                }

                scope.function.push_cmd(format!(
                    "scoreboard players operation {} {} {}= {} {}",
                    temp, temp_scoreboard, math_op.symbol(), &right_player, &right_scoreboard
                ));
                Ok(Value::IntReference {
                    player: temp.to_string(),
                    scoreboard: scope.function.name.to_string(),
                })
            }
            _ => todo!(),
        }
    }

    fn compile_addition(
        &self,
        cursor: Cursor,
        scope: &mut Scope,
        context: &Context,
        left_expr: Expression,
        right_expr: Expression,
    ) -> Result<Value, Message> {
        self.compile_math_operation(cursor, scope, context, MathOp::Addition, left_expr, right_expr)
    }

    fn compile_subtraction(
        &self,
        cursor: Cursor,
        scope: &mut Scope,
        context: &Context,
        left_expr: Expression,
        right_expr: Expression,
    ) -> Result<Value, Message> {
        self.compile_math_operation(cursor, scope, context, MathOp::Subtraction, left_expr, right_expr)
    }

    fn compile_multiplication(
        &self,
        cursor: Cursor,
        scope: &mut Scope,
        context: &Context,
        left_expr: Expression,
        right_expr: Expression,
    ) -> Result<Value, Message> {
        self.compile_math_operation(cursor, scope, context, MathOp::Multiplication, left_expr, right_expr)
    }

    fn compile_division(
        &self,
        cursor: Cursor,
        scope: &mut Scope,
        context: &Context,
        left_expr: Expression,
        right_expr: Expression,
    ) -> Result<Value, Message> {
        self.compile_math_operation(cursor, scope, context, MathOp::Division, left_expr, right_expr)
    }

    fn compile_function_declaration(
        &mut self,
        context: &Context,
        name: String,
        body: Vec<Expression>,
    ) -> Result<Value, Message> {
        let mut function = McFunction::new(name.clone());
        function.push_cmd(format!("scoreboard objectives add {} dummy", name));
        let mut scope = Scope::new(&mut function);

        for expression in body {
            self.compile_expression(&mut scope, context, expression)?;
        }

        function.push_cmd(format!("scoreboard objectives remove {}", name));

        self.functions.push(function);
        Ok(Value::FunctionReference(name))
    }

    fn compile_expression(
        &self,
        scope: &mut Scope,
        context: &Context,
        expression: Expression,
    ) -> Result<Value, Message> {
        match expression {
            Expression::VariableDeclaration {
                name,
                variable_type,
                value,
                cursor,
            } => self.compile_variable_declaration(cursor, scope, context, name, variable_type, value),
            Expression::Integer(int, cursor) => self.compile_integer(cursor, int),
            Expression::VariableAccess(identifier, cursor) => {
                self.compile_variable_access(cursor, scope, identifier)
            }
            Expression::Addition {
                left,
                right,
                cursor,
            } => self.compile_addition(cursor, scope, context, *left, *right),
            Expression::Subtraction {
                left,
                right,
                cursor,
            } => self.compile_subtraction(cursor, scope, context, *left, *right),
            Expression::Multiplication {
                left,
                right,
                cursor,
            } => self.compile_multiplication(cursor, scope, context, *left, *right),
            Expression::Division {
                left,
                right,
                cursor,
            } => self.compile_division(cursor, scope, context, *left, *right),
            _ => unimplemented!(),
        }
    }
}

pub fn compile(ast: Vec<Expression>) -> Result<String, Message> {
    let mut compiler = Compiler::new();
    compiler.compile_function_declaration(&Context { macro_target: None }, "main".to_string(), ast)?;
    Ok(compiler.functions.last().unwrap().to_string())
}
