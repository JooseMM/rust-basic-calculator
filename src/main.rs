use std::{collections::VecDeque, io};
mod utils;
mod core;

use utils::{parser, tokenize};

use crate::core::Operation;

fn main() {
    let mut input = String::new();
    let mut operation_list:VecDeque<Operation> = VecDeque::new();

    println!("Current supported operation are (+,-,*,/)");
    println!("Type your desire operation:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed at reading input");

    let token = tokenize(input);
    parser(token, &mut operation_list);

    println!("{:?}", operation_list);
}
