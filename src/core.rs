pub const COMPLEX_OPERATIONS: [char; 2] = ['*', '/'];

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Operator(char),
}
