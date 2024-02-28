#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number(String),
    AddOperator, // +
    SubOperator, // -
    MulOperator, // \*
    DivOperator, // /
    LeftParen,   // (
    RightParen,  // )
    Eof,         // \0 | \n
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        Token { token_type }
    }
}
