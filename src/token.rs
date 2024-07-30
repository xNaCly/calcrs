#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Plus,
    Minus,
    Slash,
    Asteriks,
    BraceLeft,
    BraceRight,
    Number(f64),
    String(String),
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
