pub const ALLOWED_OPERATIONS: [char; 4] = ['-', '+', '*', '/'];

#[derive(Debug)]
pub struct Operation {
    pub v1: Option<u32>,
    pub op: Option<char>,
    pub v2: Option<u32>,
    pub priority: Priority
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(u32),
    Operator(char),
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low, 
    Medium, 
    High, 
    Ultra
}
