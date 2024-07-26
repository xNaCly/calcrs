#[derive(Debug)]
pub enum TokenType {
    PLUS,
    SUB,
    DIV,
    MUL,
    NUMBER(f64),
}

#[derive(Debug)]
pub struct Token {
    pub pos: usize,
    pub t: TokenType,
}

impl Token {
    pub fn new(pos: usize, t: TokenType) -> Token {
        Token { t, pos }
    }
}
