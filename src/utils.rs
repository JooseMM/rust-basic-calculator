use crate::core::{COMPLEX_OPERATIONS, Token};

pub fn parser(mut token_list: Vec<Token>) {
    normalize(&mut token_list);
    let mut acc: i32 = 0;

    for (_,t) in token_list.iter().enumerate() {
        if let Token::Number(v) = t {
            acc += v;
        }
    }

    println!("result: {acc}");
}

pub fn tokenize(input: String, token_list: &mut Vec<Token>) {
    let mut window: [Option<char>; 2] = [None, None];

    for (_, t) in input.chars().enumerate() {
        if ['+', '-'].contains(&t) {
            window[0] = Some(t);
        } else if t.is_ascii_digit() {
            window[1] = Some(t);
            if window[0].is_none() {
                window[0] = Some('+');
                token_list.push(window_to_token(&window));
                window = [None, None];
                continue;
            }
        } else if COMPLEX_OPERATIONS.contains(&t) {
            token_list.push(Token::Operator(t));
            continue;
        }

        if window[0].is_some() && window[1].is_some() {
            token_list.push(window_to_token(&window));
            window = [None, None];
        }
    }
}

fn window_to_token(window: &[Option<char>; 2]) -> Token {
    let num: String = window.iter().map(|&c| c.unwrap()).collect();
    let num: i32 = num.parse().expect("Unable to parse the string");
    return Token::Number(num);
}

fn normalize(token_list: &mut Vec<Token>) {
    let mut v1: Option<i32> = None;
    let mut op: Option<fn(i32, i32) -> i32> = None;
    let mut start_drain: Option<usize> = None;
    let mut end_drain: Option<usize> = None;

    for (i, token) in token_list.iter().enumerate() {
        match token {
            Token::Number(v2) => {
                if v1.is_some() && op.is_some() {
                    v1 = Some(op.unwrap()(v1.unwrap(), *v2));
                    if start_drain.is_some() {
                        end_drain = Some(i);
                    } else {
                        start_drain = Some(i - 2);
                    }
                } else {
                    v1 = Some(*v2);
                }
            }
            Token::Operator(o) => {
                if *o == '*' {
                    op = Some(multiplication);
                } else {
                    op = Some(division);
                }
            }
        };
    }
    match end_drain {
        Some(end) => {
            if let Some(start) = start_drain && let Some(result) = v1 {
                token_list.drain(start..end);
                token_list[start] = Token::Number(result);
            }
        }
        None => (),
    }
}

fn multiplication(a: i32, b: i32) -> i32 {
    a * b
}

fn division(a: i32, b: i32) -> i32 {
    a / b
}
