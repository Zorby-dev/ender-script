use std::collections::HashMap;

use crate::{lexer::{Token, TokenType}, message::Message, parser::{Node, NodeType}, util::Position};

/*
basic types
 - byte        (byte,           i8)
 - short       (short,         i16)
 - int         (int,           i32)
 - long        (long,          i64)
 - float       (float, number, f32)
 - double      (double,        f64)
 - string      (str)
 - list        (array)
 - compound    (object)
 - byte array  (array<i8>)
 - int array   (array<i32>)
 - long array  (array<i64>)
 - boolean     (true | false)
    - true     (1b)
    - false    (0b)
key types
 - function
 - class
 - module
compound types
*/

#[derive(Clone, Debug)]
enum Type {
    //Byte(i8),
    //Short(i16),
    Int(i32),
    IntPointer(String),
    //Long(i64),
    //Float(f32),
    //Double(f64)
}

use self::Type::*;

#[derive(Clone, Debug)]
pub struct Value {
    typ: Type,
    start_pos: Option<Position>,
    end_pos: Option<Position>,
}
impl Value {
    fn new(typ: Type) -> Self {
        Value {
            typ,
            start_pos: None,
            end_pos: None,
        }
    }

    fn set_pos(mut self, start_pos: &Position, end_pos: &Position) -> Self {
        self.start_pos = Some(start_pos.clone());
        self.end_pos = Some(end_pos.clone());
        self
    }
}

#[derive(Clone)]
pub struct Path {
    segments: Vec<String>
}
impl ToString for Path {
    fn to_string(&self) -> String {
        format!("{}:{}", self.segments[0], self.segments[1..].join("/"))
    }
}
impl Path {
    fn new(path: &str) -> Self {
        let segments: Vec<String> = path.split('.').map(|x| x.to_owned()).collect();
        assert!(segments.len() >= 1);
        Self { segments }
    }

    fn namespace(&self) -> &String {
        &self.segments[0]
    }
}

#[derive(Clone)]
pub struct BinFile {
    pub text: String,
    pub path: Path
}
impl BinFile {
    pub fn new(path: Path) -> Self {
        BinFile {
            text: String::new(),
            path
        }
    }
    pub fn push(&mut self, line: &str) -> &Self {
        self.text += &format!("{}\n", line);
        self
    }
}

pub struct CompileRegister {
    result: Option<Result<Value, Message>>,
    last_registered_advance_count: usize,
    advance_count: usize,
    to_reverse_count: usize,
}

impl CompileRegister {
    pub fn new() -> Self {
        CompileRegister {
            result: None,
            last_registered_advance_count: 0,
            advance_count: 0,
            to_reverse_count: 0,
        }
    }

    pub fn register_advancement(&mut self) -> &Self {
        self.last_registered_advance_count += 1;
        self.advance_count += 1;
        return self;
    }

    pub fn register(&mut self, result: &CompileResult) -> Result<Value, Message> {
        self.last_registered_advance_count = result.advance_count;
        self.advance_count += result.advance_count;
        match &result.result {
            Ok(node) => Ok(node.clone()),
            Err(error) => {
                self.result = Some(Err(error.clone()));
                return Err(error.clone());
            }
        }
    }

    pub fn try_register(&mut self, result: &CompileResult) -> Result<Value, Message> {
        match &result.result {
            Ok(_) => self.register(result),
            Err(error) => {
                self.to_reverse_count = result.advance_count;
                return Err(error.clone());
            }
        }
    }

    pub fn success(&mut self, value: Value) -> CompileResult {
        self.result = Some(Ok(value));
        return CompileResult {
            result: self.result.clone().unwrap(),
            last_registered_advance_count: self.last_registered_advance_count,
            advance_count: self.advance_count,
            to_reverse_count: self.to_reverse_count,
        };
    }

    pub fn failure(&mut self, error: Message) -> CompileResult {
        self.result = Some(Err(error));
        return CompileResult {
            result: self.result.clone().unwrap(),
            last_registered_advance_count: self.last_registered_advance_count,
            advance_count: self.advance_count,
            to_reverse_count: self.to_reverse_count,
        };
    }

    pub fn pack(&mut self) -> CompileResult {
        match &self.result {
            Some(result) => CompileResult {
                result: result.clone(),
                last_registered_advance_count: self.last_registered_advance_count,
                advance_count: self.advance_count,
                to_reverse_count: self.to_reverse_count,
            },
            None => {
                panic!();
            }
        }
    }

    pub fn failed(&self) -> bool {
        match self.result {
            Some(Err(_)) => true,
            Some(_) => false,
            None => false,
        }
    }

    pub fn succeeded(&self) -> bool {
        match self.result {
            Some(Ok(_)) => true,
            Some(_) => false,
            None => false,
        }
    }
}

pub struct CompileResult {
    result: Result<Value, Message>,
    last_registered_advance_count: usize,
    advance_count: usize,
    to_reverse_count: usize,
}

impl CompileResult {
    pub fn to_register(&self) -> CompileRegister {
        CompileRegister {
            advance_count: self.advance_count,
            last_registered_advance_count: self.last_registered_advance_count,
            to_reverse_count: self.to_reverse_count,
            result: Some(self.result.clone()),
        }
    }
}

struct SymbolTable<'a> {
    symbols: HashMap<String, Value>,
    parent: Option<&'a SymbolTable<'a>>
}
impl<'a> SymbolTable<'a> {
    fn new(parent: Option<&'a SymbolTable<'a>>) -> Self {
        SymbolTable { symbols: HashMap::new(), parent}
    }

    fn add(&self, name: String, value: Value) -> Value {
        self.symbols.insert(name, value);
        value
    }
}

struct Compiler {
    main_file: BinFile,
    helper_files: Vec<BinFile>,
}
impl Compiler {
    fn new(path: &str) -> Self {
        Compiler {main_file: BinFile::new(Path::new(path)), helper_files: Vec::new()}
    }

    fn add_int_to_int(&self, first: &i32, second: &i32) -> Value {
        Value::new(Int(first + second))
    }

    fn add_int_to_int_ptr(&mut self, ptr: &String, val: &i32) -> Value {
        self.main_file.push(&format!("scoreboard players add .{} .es-storage {}", ptr, val));
        Value::new(IntPointer(ptr.to_string()))
    }
    
    fn add(&mut self, first: &Value, second: &Value) -> CompileResult {
        match &first.typ {
            Int(first_val) => match &second.typ {
                Int(second_val) => CompileRegister::new().success(self.add_int_to_int(first_val, second_val)),
                IntPointer(ptr) => CompileRegister::new().success(self.add_int_to_int_ptr(&ptr, first_val)),
            },
            IntPointer(ptr) => match &second.typ {
                Int(first_val) => CompileRegister::new().success(self.add_int_to_int_ptr(&ptr, first_val)),
                IntPointer(_) => todo!(),
            },
        }
    }
    
    /*fn sub(&self, first: &Value, second: &Value) -> CompileResult {
        match first.typ {
            Int(first_val) => match second.typ {
                Int(second_val) => CompileRegister::new().success(Value::new(Int(first_val - second_val))),
            },
        }
    }
    
    fn mul(&self, first: &Value, second: &Value) -> CompileResult {
        match first.typ {
            Int(first_val) => match second.typ {
                Int(second_val) => CompileRegister::new().success(Value::new(Int(first_val * second_val))),
            },
        }
    }
    
    fn div(&self, first: &Value, second: &Value) -> CompileResult {
        match first.typ {
            Int(first_val) => match second.typ {
                Int(second_val) => CompileRegister::new().success(Value::new(Int(first_val / second_val))),
            },
        }
    }*/

    fn visit_number(&self, token: &Token, node: &Node) -> CompileResult {
        match token.token_type {
            TokenType::Int(value) => CompileRegister::new().success(Value::new(Int(value)).set_pos(&node.start_pos, &node.end_pos)),
            _                         => panic!("Not a Number")
        }
    }
    
    fn visit_bin_op(&mut self, left_node: &Node, op: &Token, right_node: &Node) -> CompileResult {
        let mut reg = CompileRegister::new();
        let left: Value;
        match reg.register(&self.visit(left_node)) {
            Ok(value) => left = value,
            Err(_) => return reg.pack(),
        }
        let right: Value;
        match reg.register(&self.visit(right_node)) {
            Ok(value) => right = value,
            Err(_) => return reg.pack(),
        }
        match op.token_type {
            TokenType::Plus => {
                match reg.register(&self.add(&left, &right)) {
                    Ok(value) => reg.success(value),
                    Err(_) => reg.pack(),
                }
            },
            TokenType::Minus => /*{
                match reg.register(&self.sub(&left, &right)) {
                    Ok(value) => reg.success(value),
                    Err(_) => reg.pack(),
                }
            }*/todo!(),
            TokenType::Mul => /*{
                match reg.register(&self.mul(&left, &right)) {
                    Ok(value) => reg.success(value),
                    Err(_) => reg.pack(),
                }
            }*/todo!(),
            TokenType::Div => /*{
                match reg.register(&self.div(&left, &right)) {
                    Ok(value) => reg.success(value),
                    Err(_) => reg.pack(),
                }
            }*/todo!(),
            _ => panic!("Not an operator")
        }
    }

    fn visit_var_declaration(&mut self, name_token: &Token, typ_option: &Option<Token>, value_option: &Option<Box<Node>>) -> CompileResult {
        let mut reg = CompileRegister::new();
        match &name_token.token_type {
            TokenType::ID(name) => {
                match typ_option {
                    Some(typ_token) => match &typ_token.token_type {
                        TokenType::ID(typ) => {
                            match value_option {
                                Some(value_box) => {
                                    let value_node: Node = *(value_box.to_owned());
                                    let value: Value;
                                    match reg.register(&self.visit(&value_node)) {
                                        Ok(val) => value = val,
                                        Err(_) => return reg.pack(),
                                    }
                                    match value.typ {
                                        Int(val) => {
                                            return reg.success(Value::new(IntPointer()))
                                        },
                                        IntPointer(val) => todo!(),
                                    }
                                },
                                None => todo!(),
                            }
                        },
                        _ => {
                            panic!("Not an ID")
                        }
                    },
                    None => todo!(),
                }
            },
            _ => {
                panic!("Not an ID")
            }
        }
    }

    #[allow(unused)]
    fn visit(&mut self, node: &Node) -> CompileResult {
        match &node.node_type {
            NodeType::VarAccess { name } => todo!(),
            NodeType::VarDeclaration { name, typ, value } => self.visit_var_declaration(name, typ, value),
            NodeType::VarAssign { name, value } => todo!(),
            NodeType::FunctionDeclaration {
                name,
                args,
                typ,
                expr,
            } => todo!(),
            NodeType::FunctionCall { name, args } => todo!(),
            NodeType::Number { token } => self.visit_number(token, node),
            NodeType::String { token } => todo!(),
            NodeType::BinOp {
                left_node,
                op,
                right_node,
            } => self.visit_bin_op(left_node, op, right_node),
            NodeType::UnaryOp { op, node } => todo!(),
            NodeType::If {
                condition,
                expr,
                else_case,
            } => todo!(),
            NodeType::While { condition, expr } => todo!(),
            NodeType::Statements { statements } => todo!(),
            NodeType::Arguments { args } => todo!(),
            NodeType::Parameter { name, typ, value } => todo!(),
            NodeType::Parameters { params } => todo!(),
        }
    }

    fn bin_files(&self) -> Vec<BinFile> {
        let mut bin_files = self.helper_files.to_vec();
        bin_files.push(self.main_file.clone());
        bin_files
    }
}

pub fn compile(node: Node, path: &str) -> Result<Vec<BinFile>, Message> {
    let mut compiler = Compiler::new(path);
    let result = compiler.visit(&node);
    match result.result {
        Ok(value) => { println!("{:?}", value); return Ok(compiler.bin_files()); },
        Err(error) => Err(error),
    }
}
