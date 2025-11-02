use std::collections::VecDeque;

use crate::core::{ALLOWED_OPERATIONS, Operation, Token};

pub fn parser(mut tokens: Vec<Token>, ops_stack: &mut VecDeque<Operation>) {
    loop {
        extract_operation(&mut tokens, ops_stack, '/');
        extract_operation(&mut tokens, ops_stack, '*');
        extract_operation(&mut tokens, ops_stack, '+');
        extract_operation(&mut tokens, ops_stack, '-');

        if tokens.is_empty() {
            break;
        }
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
            if ALLOWED_OPERATIONS.contains(&c) {
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
    raw_operator: char,
) {
    let operator_i = match tokens
        .iter()
        .position(|o| o == &Token::Operator(raw_operator))
    {
        Some(i) => i,
        None => return,
    };

    tokens.remove(operator_i);

    let mut operation = Operation {
        v1: None,
        op: Some(raw_operator),
        v2: None,
    };

    let v2_i = (operator_i + 1) - 1;
    match tokens.get(v2_i) {
        Some(next_digit) => match next_digit {
            Token::Number(v) => {
                operation.v2 = Some(*v);
                tokens.remove(v2_i);
            }
            Token::Operator(op) => {
                panic!("Expected a number after {op} operator");
            }
        },
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
        Token::Operator(op) => {
            panic!("Expected a number before {op} operator");
        }
    }

    return add_operation(stack_operation, operation);
}

fn add_operation(stack_operation: &mut VecDeque<Operation>, operation: Operation) {
    let op = match operation.op {
        Some(o) => o,
        None => return,
    };
}
