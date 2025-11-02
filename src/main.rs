use std::io;
mod utils;
mod core;

use utils::{parser, tokenize};

fn main() {
    let mut input = String::new();
    println!("Current supported operation are (+,-,*,/)");
    println!("Type your desire operation:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed at reading input");

    let tokens = tokenize(input);
    parser(tokens);
}
