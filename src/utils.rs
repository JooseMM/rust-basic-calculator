use crate::core::{ALLOWED_OPERATIONS, Operation, Priority, Token};

pub fn parser(mut tokens: Vec<Token>) {
    let mut ops_stack: Vec<Operation> = vec![];

    println!("Before: {:?}", tokens);

    extract_operation(&mut tokens, &mut ops_stack, '*', Priority::Medium);

    println!("After: {:?}", tokens);
    println!("stack: {:?}", ops_stack)
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
    stack_operation: &mut Vec<Operation>,
    raw_operator: char,
    priority: Priority,
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
        priority,
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

fn add_operation(stack_operation: &mut Vec<Operation>, operation: Operation) {
    let position = stack_operation
        .iter()
        .position(|o| o.priority < operation.priority)
        .unwrap_or(0);

    if position + 1 < stack_operation.len() {
        return stack_operation.insert(position + 1, operation);
    }

    stack_operation.push(operation);
}
