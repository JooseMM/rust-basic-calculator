use std::collections::VecDeque;

use crate::core::{BASIC_OPERATIONS, COMPLEX_OPERATIONS, Operation, Token};

pub fn parser(mut tokens: Vec<Token>, ops_stack: &mut VecDeque<Operation>) {
    while !tokens.is_empty() {
        extract_operation(&mut tokens, ops_stack, &COMPLEX_OPERATIONS);
        extract_operation(&mut tokens, ops_stack, &BASIC_OPERATIONS);
    }
}

pub fn tokenize(input: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else if c.is_ascii_digit() {
            current.clear();
            while let Some(&ch) = chars.peek() {
                if ch.is_ascii_digit() {
                    current.push(ch);
                    chars.next();
                } else {
                    break;
                }
            }

            let digit = current.clone().to_string();
            let digit = digit.parse::<u32>().expect("Unable to parse digit");
            tokens.push(Token::Number(digit));
        } else {
            if BASIC_OPERATIONS.contains(&c) || COMPLEX_OPERATIONS.contains(&c) {
                tokens.push(Token::Operator(c));
                chars.next();
            }
        }
    }

    return tokens;
}

pub fn extract_operation(
    tokens: &mut Vec<Token>,
    stack_operation: &mut VecDeque<Operation>,
    raw_operators: &[char;2],
) {
    let operator_i = match tokens
        .iter()
        .position(|o| o == &Token::Operator(raw_operators[0]) || o == &Token::Operator(raw_operators[1]))
    {
        Some(i) => i,
        None => return
    };

    let operator = match tokens[operator_i] {
        Token::Operator(o) => o,
        Token::Number(_) => return
    };

    
    tokens.remove(operator_i);
    let mut operation = Operation {
        v1: None,
        v2: None,
        op: Some(operator)
    };

    let v2_i = operator_i;
    match tokens.get(v2_i) {
        Some(next_digit) => match next_digit {
            Token::Number(v) => {
                operation.v2 = Some(*v);
            }
            Token::Operator(operator) => {
                panic!("Expected a number after {operator} operator");
            }
        }
        None => (),
    }

    if operator_i == 0 {
        return add_operation(stack_operation, operation);
    }

    let v1_i = operator_i - 1;
    match tokens[v1_i] {
        Token::Number(v) => {
            operation.v1 = Some(v);
            tokens.remove(v1_i);
        }
        Token::Operator(operator) => {
            panic!("Expected a number before {operator} operator");
        }
    }

    return add_operation(stack_operation, operation);
}

fn add_operation(stack_operation: &mut VecDeque<Operation>, operation: Operation) {
    let op = match operation.op {
        Some(o) => o,
        None => return,
    };

    if COMPLEX_OPERATIONS.contains(&op) {
        return stack_operation.push_front(operation);
    } else if BASIC_OPERATIONS.contains(&op) {
        return stack_operation.push_back(operation);
    }
}
