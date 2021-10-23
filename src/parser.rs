use crate::lexer::{Keyword, Token, TokenType};
use crate::util::{Error, Position};

#[allow(nonstandard_style)]
fn InvalidSyntaxError(details: String, start_pos: Position, end_pos: Position) -> Error {
    return Error { name: String::from("Invalid Syntax"), details, start_pos, end_pos }
}

#[derive(Clone, Debug)]
enum NodeType {
    VarAccess {name: Token},
    VarDeclaration {name: Token, typ: Option<Token>, value: Option<Box<Node>>},
    VarAssign {name: Token, value: Box<Node>},
    FunctionDeclaration {name: Token, args: Box<Node>, expr: Box<Node>},
    FunctionCall {name: Token, args: Box<Node>},
    Number {token: Token},
    String {token: Token},
    BinOp {left_node: Box<Node>, op: Token, right_node: Box<Node>},
    UnaryOp {op: Token, node: Box<Node>},
    If {condition: Box<Node>, expr: Box<Node>, else_case: Option<Box<Node>>},
    While {condition: Box<Node>, expr: Box<Node>},
    Statements {statements: Vec<Box<Node>>},
    Arguments {args: Vec<Box<Node>>},
    Parameters {params: Vec<Box<Node>>}
}

#[derive(Clone, Debug)]
pub struct Node {
    node_type: NodeType,
    start_pos: Position,
    end_pos: Position
}

impl Node {
    pub fn var_access(name: Token) -> Self {
        Node { node_type: NodeType::VarAccess{name: name.clone()}, start_pos: name.start_pos.clone(), end_pos: name.end_pos.clone() }
    }
    pub fn var_declaration(name: Token, typ: Option<Token>, value: Option<Node>) -> Self {
        match value {
            Some(value) => Node { node_type: NodeType::VarDeclaration{name: name.clone(), typ, value: Some(Box::new(value))}, start_pos: name.start_pos.clone(), end_pos: name.end_pos.clone() },
            None => Node { node_type: NodeType::VarDeclaration{name: name.clone(), typ, value: None}, start_pos: name.start_pos.clone(), end_pos: name.end_pos.clone() }
        }
    }
    pub fn var_assign(name: Token, value: &Node) -> Self {
        Node { node_type: NodeType::VarAssign{name: name.clone(), value: Box::new(value.clone())}, start_pos: name.start_pos.clone(), end_pos: name.end_pos.clone() }
    }
    pub fn function_declaration(start_pos: &Position, name: Token, args: &Node, expr: &Node) -> Self {
        Node { node_type: NodeType::FunctionDeclaration{name: name.clone(), args: Box::new(args.clone()), expr: Box::new(expr.clone())}, start_pos: start_pos.clone(), end_pos: expr.end_pos.clone()}
    }
    pub fn function_call(start_pos: &Position, name: Token, args: &Node) -> Self {
        Node { node_type: NodeType::FunctionCall{name: name.clone(), args: Box::new(args.clone())}, start_pos: start_pos.clone(), end_pos: args.end_pos.clone()}
    }
    pub fn number(token: Token) -> Self {
        Node { node_type: NodeType::Number{token: token.clone()}, start_pos: token.start_pos.clone(), end_pos: token.end_pos.clone() }
    }
    pub fn string(token: Token) -> Self {
        Node { node_type: NodeType::String{token: token.clone()}, start_pos: token.start_pos.clone(), end_pos: token.end_pos.clone() }
    }
    pub fn bin_op(left_node: &Node, op: Token, right_node: &Node) -> Self {
        Node { node_type: NodeType::BinOp{left_node: Box::new(left_node.clone()), op: op, right_node: Box::new(right_node.clone())}, start_pos: left_node.start_pos.clone(), end_pos: right_node.end_pos.clone() }
    }
    pub fn unary_op(op: Token, node: &Node) -> Self {
        Node { node_type: NodeType::UnaryOp{op: op.clone(), node: Box::new(node.clone())}, start_pos: op.start_pos.clone(), end_pos: node.end_pos.clone() }
    }
    pub fn r#if(start_pos: &Position, condition: &Node, expr: &Node, else_case: Option<Node>) -> Self {
        match else_case {
            Some(else_case) => Node { node_type: NodeType::If{condition: Box::new(condition.clone()), expr: Box::new(expr.clone()), else_case: Some(Box::new(else_case.clone()))}, start_pos: start_pos.clone(), end_pos: else_case.end_pos.clone()},
            None => Node { node_type: NodeType::If{condition: Box::new(condition.clone()), expr: Box::new(expr.clone()), else_case: None}, start_pos: start_pos.clone(), end_pos: expr.end_pos.clone()}
        }
    }
    pub fn r#while(start_pos: &Position, condition: &Node, expr: &Node) -> Self {
        Node { node_type: NodeType::While{condition: Box::new(condition.clone()), expr: Box::new(expr.clone())}, start_pos: start_pos.clone(), end_pos: expr.end_pos.clone()}
    }
    pub fn statements(start_pos: &Position, statements: Vec<Node>, end_pos: &Position) -> Self {
        let mut boxed_statements: Vec<Box<Node>> = Vec::new();
        for statement in statements {
            boxed_statements.push(Box::new(statement));
        }
        Node { node_type: NodeType::Statements{statements: boxed_statements}, start_pos: start_pos.clone(), end_pos: end_pos.clone() }
    }
    pub fn arguments(start_pos: &Position, arguments: Vec<Node>, end_pos: &Position) -> Self {
        let mut boxed_arguments: Vec<Box<Node>> = Vec::new();
        for statement in arguments {
            boxed_arguments.push(Box::new(statement));
        }
        Node { node_type: NodeType::Arguments{args: boxed_arguments}, start_pos: start_pos.clone(), end_pos: end_pos.clone() }
    }
    pub fn parameters(start_pos: &Position, parameters: Vec<Node>, end_pos: &Position) -> Self {
        let mut boxed_parameters: Vec<Box<Node>> = Vec::new();
        for statement in parameters {
            boxed_parameters.push(Box::new(statement));
        }
        Node { node_type: NodeType::Parameters{params: boxed_parameters}, start_pos: start_pos.clone(), end_pos: end_pos.clone() }
    }
}

#[derive(Clone)]
pub enum Result {
    Success(Node),
    Failure(Error)
}

use self::Result::*;

pub struct ParseRegister {
    result: Option<Result>,
    last_registered_advance_count: usize,
    advance_count: usize,
    to_reverse_count: usize
}

impl ParseRegister {
    pub fn new() -> Self {
        ParseRegister { result: None, last_registered_advance_count: 0, advance_count: 0, to_reverse_count: 0 }
    }

    pub fn register_advancement(&mut self) -> &Self {
        self.last_registered_advance_count += 1;
        self.advance_count += 1;
        return self;
    }

    pub fn register(&mut self, result: &ParseResult) -> Result {
        self.last_registered_advance_count = result.advance_count;
        self.advance_count += result.advance_count;
        match &result.result {
            Success(node) => Success(node.clone()),
            Failure(error) => {
                self.result = Some(Failure(error.clone()));
                return Failure(error.clone());
            }
        }
    }

    pub fn try_register(&mut self, result: &ParseResult) -> Result {
        match &result.result {
            Success(_) => self.register(result),
            Failure(error) => {
                self.to_reverse_count = result.advance_count;
                return Failure(error.clone());
            }
        }
    }

    pub fn success(&mut self, node: Node) -> ParseResult {
        self.result = Some(Success(node));
        return ParseResult {result: self.result.clone().unwrap(), last_registered_advance_count: self.last_registered_advance_count, advance_count: self.advance_count, to_reverse_count: self.to_reverse_count};
    }

    pub fn failure(&mut self, error: Error) -> ParseResult {
        self.result = Some(Failure(error));
        return ParseResult {result: self.result.clone().unwrap(), last_registered_advance_count: self.last_registered_advance_count, advance_count: self.advance_count, to_reverse_count: self.to_reverse_count};
    }

    pub fn pack(&mut self) -> ParseResult {
        match &self.result {
            Some(result) => ParseResult {result: result.clone(), last_registered_advance_count: self.last_registered_advance_count, advance_count: self.advance_count, to_reverse_count: self.to_reverse_count},
            None => { panic!(); }
        }
    }

    pub fn failed(&self) -> bool {
        match self.result {
            Some(Failure(_)) => true,
            Some(_) => false,
            None => false
        }
    }

    pub fn succeeded(&self) -> bool {
        match self.result {
            Some(Success(_)) => true,
            Some(_) => false,
            None => false
        }
    }
}

struct State<'a> {
    index: usize,
    current_token: &'a Token,
    tokens: &'a Vec<Token>
}

pub struct ParseResult {
    result: Result,
    last_registered_advance_count: usize,
    advance_count: usize,
    to_reverse_count: usize
}

impl ParseResult {
    pub fn to_register(&self) -> ParseRegister {
        ParseRegister {
            advance_count: self.advance_count,
            last_registered_advance_count: self.last_registered_advance_count,
            to_reverse_count: self.to_reverse_count,
            result: Some(self.result.clone())
        }
    }
}

fn update_current_token(state: &mut State) {
    if state.index < state.tokens.len() {
        state.current_token = &state.tokens[state.index];
    }
}

fn advance<'a>(mut state: &'a mut State) -> &'a Token {
    state.index += 1;
    update_current_token(&mut state);
    return state.current_token;
}

fn reverse<'a>(mut state: &'a mut State, amount: usize) -> &'a Token {
    state.index -= amount;
    update_current_token(state);
    return state.current_token;
}


fn if_expression(mut state: &mut State) -> ParseResult {
    todo!();
}

fn while_expression(mut state: &mut State) -> ParseResult {
    todo!();
}

fn function_declaration(mut state: &mut State) -> ParseResult {
    todo!();
}

fn arguments(state: &mut State) -> ParseResult {
    let mut reg = ParseRegister::new();
    let mut args: Vec<Node> = Vec::new();
    let start_pos = state.current_token.start_pos.clone();

    while !(state.current_token.is(TokenType::RParen)) {
        match reg.register(&expression(state)) {
            Success(node) => { args.push(node); },
            Failure(_) => { return reg.pack(); }
        }

        if !(state.current_token.is(TokenType::Comma)) {
            break;
        }

        reg.register_advancement();
        advance(state);
    }

    if !(state.current_token.is(TokenType::RParen)) {
        return reg.failure(InvalidSyntaxError(String::from("Expected ')'"), state.current_token.start_pos.clone(), state.current_token.end_pos.clone()))
    }

    reg.register_advancement();
    advance(state);

    return reg.success(Node::arguments(&start_pos, args, &state.current_token.end_pos))
}

fn factor(state: &mut State) -> ParseResult {
    let mut reg = ParseRegister::new();
    let token = state.current_token.clone();

    match token.token_type {
        TokenType::LParen => {
            reg.register_advancement();
            advance(state);
            let expr: Node;
            match reg.register(&expression(state)) {
                Success(node) => { expr = node; },
                Failure(_) => { return reg.pack(); }
            }
            match state.current_token.token_type {
                TokenType::RParen => {
                    reg.register_advancement();
                    advance(state);
                    return reg.success(expr);
                },
                _ => {
                    return reg.failure(InvalidSyntaxError(String::from("Expected ')'"), state.current_token.start_pos.clone(), state.current_token.end_pos.clone()));
                }
            }
        },
        TokenType::Keyword(Keyword::If) => {
            let expr: Node;
            match reg.register(&if_expression(state)) {
                Success(node) => { expr = node; },
                Failure(_) => { return reg.pack(); }
            }
            return reg.success(expr);
        },
        TokenType::Keyword(Keyword::While) => {
            let expr: Node;
            match reg.register(&while_expression(state)) {
                Success(node) => { expr = node; },
                Failure(_) => { return reg.pack(); }
            }
            return reg.success(expr);
        },
        TokenType::Keyword(Keyword::Function) => {
            let expr: Node;
            match reg.register(&function_declaration(state)) {
                Success(node) => { expr = node; },
                Failure(_) => { return reg.pack(); }
            }
            return reg.success(expr);
        },
        TokenType::ID(_) => {
            let var_name = state.current_token.clone();

            reg.register_advancement();
            advance(state);

            match state.current_token.token_type {
                TokenType::EQ => {
                    reg.register_advancement();
                    advance(state);

                    let expr: Node;
                    match reg.register(&expression(state)) {
                        Success(node) => { expr = node; },
                        Failure(_) => { return reg.pack(); }
                    }
                    return reg.success(Node::var_assign(var_name, &expr));
                },
                TokenType::LParen => {
                    reg.register_advancement();
                    advance(state);

                    let args: Node;
                    match reg.register(&arguments(state)) {
                        Success(node) => { args = node; },
                        Failure(_) => { return reg.pack(); }
                    }

                    return reg.success(Node::function_call(&var_name.start_pos, var_name.clone(), &args))
                },
                TokenType::Colon => {
                    reg.register_advancement();
                    advance(state);

                    let var_type: Option<Token>;
                    let value: Option<Node>;
                    match state.current_token.token_type {
                        TokenType::ID(_) => {
                            var_type = Some(state.current_token.clone());

                            reg.register_advancement();
                            advance(state);

                            match state.current_token.token_type {
                                TokenType::EQ => {
                                    reg.register_advancement();
                                    advance(state);

                                    let expr: Node;
                                    match reg.register(&expression(state)) {
                                        Success(node) => { expr = node; },
                                        Failure(_) => { return reg.pack(); }
                                    }
                                    value = Some(expr);
                                },
                                _ => {
                                    value = None;
                                }
                            }
                        },
                        TokenType::EQ => {
                            var_type = None;

                            reg.register_advancement();
                            advance(state);

                            let expr: Node;
                            match reg.register(&expression(state)) {
                                Success(node) => { expr = node; },
                                Failure(_) => { return reg.pack(); }
                            }
                            value = Some(expr);
                        },
                        _ => {
                            return reg.failure(InvalidSyntaxError(String::from("Expected variable type or '='"), state.current_token.start_pos.clone(), state.current_token.end_pos.clone()));
                        }
                    }
                    return reg.success(Node::var_declaration(var_name, var_type, value));
                }
                _ => { return reg.success(Node::var_access(var_name)); }
            }
        },
        TokenType::Minus => {
            reg.register_advancement();
            advance(state);

            let node: Node;
            match reg.register(&factor(state)) {
                Success(nod) => { node = nod; },
                Failure(_) => { return reg.pack(); }
            }
            return reg.success(node);
        },
        TokenType::Int(_) | TokenType::Float(_) => {
            reg.register_advancement();
            advance(state);
            return reg.success(Node::number(token.clone()))
        },
        TokenType::String(_) => {
            todo!();
        },
        _ => {
            return reg.failure(InvalidSyntaxError(String::from("Expected expression"), token.start_pos, token.end_pos))
        }
    }
}

fn bin_op(mut state: &mut State, func: &dyn Fn(&mut State) -> ParseResult, ops: Vec<TokenType>) -> ParseResult {
    let mut reg = ParseRegister::new();
    let mut left: Node;
    match reg.register(&func(&mut state)) {
        Success(node) => { left = node; },
        Failure(_) => { return reg.pack(); },
    }

    while ops.contains(&state.current_token.token_type) {
        let op_token = state.current_token;
        reg.register_advancement();
        advance(state);
        let right: Node;
        match reg.register(&func(&mut state)) {
            Success(node) => { right = node; },
            Failure(_) => { return reg.pack(); }
        }
        left = Node::bin_op(&left, op_token.clone(), &right);
    }

    return reg.success(left);
}

fn term(state: &mut State) -> ParseResult {
    bin_op(state, &factor, Vec::from([TokenType::Plus, TokenType::Minus]))
}

fn arithmetic_expression(state: &mut State) -> ParseResult {
    bin_op(state, &term, Vec::from([TokenType::Plus, TokenType::Minus]))
}

fn comparison_expression(mut state: &mut State) -> ParseResult {
    let mut reg = ParseRegister::new();

    match state.current_token.token_type {
        TokenType::Not | TokenType::Keyword(Keyword::Not) => {
            let op_token = state.current_token.clone();
            reg.register_advancement();
            advance(state);

            let node: Node;
            match reg.register(&expression(state)) {
                Success(nod) => { node = nod; },
                Failure(_) => { return reg.pack(); }
            }

            return reg.success(Node::unary_op(op_token, &node));
        },
        _ => {
            let node: Node;
            match reg.register(&bin_op(&mut state, &arithmetic_expression, Vec::from([TokenType::EE, TokenType::NE, TokenType::LT, TokenType::GT, TokenType::LTE, TokenType::GTE]))) {
                Success(nod) => { node = nod; },
                Failure(_) => { return reg.pack(); }
            }

            return reg.success(node);
        }
    }
}

fn expression(mut state: &mut State) -> ParseResult {
    bin_op(&mut state, &comparison_expression, Vec::from([TokenType::And, TokenType::Or, TokenType::Keyword(Keyword::And), TokenType::Keyword(Keyword::Or)]))
}

fn statements(mut state: &mut State) -> ParseResult {
    let mut reg = ParseRegister::new();
    let mut statements: Vec<Node> = Vec::new();
    let start_pos = state.current_token.start_pos.clone();

    while let TokenType::NL = state.current_token.token_type {
        reg.register_advancement();
        advance(&mut state);
    }

    let statement;
    match reg.register(&expression(&mut state)) {
        Success(node) => statement = node,
        Failure(_) => return reg.pack()
    }
    statements.push(statement);

    let mut more_statements = true;

    loop {
        let mut nl_count: usize = 0;

        while state.current_token.is(TokenType::NL) {
            reg.register_advancement();
            advance(state);
            nl_count += 1;
        }
        if nl_count == 0 {
            more_statements = false;
        }
        if !more_statements {
            break;
        }
        let statement;
        match reg.try_register(&expression(&mut state)) {
            Success(node) => statement = node,
            Failure(_) => {
                reverse(state, reg.to_reverse_count);
                more_statements = false;
                continue;
            }
        }
        statements.push(statement);
    }

    return reg.success(Node::statements(&start_pos, statements, &state.current_token.end_pos));
}

pub fn parse(tokens: Vec<Token>) -> Result {
    let mut state = State {index: 0, tokens: &tokens, current_token: &tokens[0]};

    let result = statements(&mut state);
    if let Success(_) = result.result {
        if !state.current_token.is(TokenType::EOF) {
            return result.to_register().failure(InvalidSyntaxError(String::from(""), state.current_token.start_pos.clone(), state.current_token.end_pos.clone())).result;
        }
    }
    return result.result;
}