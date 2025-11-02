use crate::core::{COMPLEX_OPERATIONS, Token};

pub fn parser(tokens: &mut Vec<Token>) {
    println!("{:?}", tokens);
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
