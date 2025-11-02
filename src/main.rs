use std::io;
mod utils;
mod core;

use utils::{parser, tokenize};
use core::{Token};

fn main() {
    let mut input = String::new();
    let mut token_list: Vec<Token> = Vec::new();

    println!("Current supported operation are (+,-,*,/)");
    println!("Type your desire operation:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed at reading input");

    tokenize(input, &mut token_list);
    parser(token_list);
}
