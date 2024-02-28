use super::token::{Token, TokenType};

#[derive(Clone, Copy, Debug, PartialEq)]
enum LexerStatus {
    Initial,
    IntNumber,   // 整数
    FloatNumber, // 小数
}

pub struct Lexer {
    source: Vec<char>,
    current: usize,
    status: LexerStatus,
}

impl Lexer {
    pub fn new(text: &str) -> Self {
        Lexer {
            source: text.chars().collect(),
            current: 0,
            status: LexerStatus::Initial,
        }
    }

    pub fn get_token(&mut self) -> Result<Token, &'static str> {
        self.skip_whitespace();
        if self.is_at_end() {
            return Ok(Token::new(TokenType::Eof));
        }

        let start = self.current;
        let c = self.advance();

        self.status = LexerStatus::Initial;        

        match c {
            c if c.is_numeric() => self.number(start),
            '.' => self.dot(start),
            '+' => Ok(Token::new(TokenType::AddOperator)),
            '-' => Ok(Token::new(TokenType::SubOperator)),
            '*' => Ok(Token::new(TokenType::MulOperator)),
            '/' => Ok(Token::new(TokenType::DivOperator)),
            '(' => Ok(Token::new(TokenType::LeftParen)),
            ')' => Ok(Token::new(TokenType::RightParen)),
            _ => Err("Syntax error."),
        }
    }

    fn advance(&mut self) -> char {
        if self.is_at_end() {
            '\0' // 返回一个明显的结束字符，表示到达了源代码的末尾
        } else {
            let c = self.source[self.current];
            self.current += 1;
            c
        }
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && self.peek().is_whitespace() {
            self.advance();
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source[self.current];
    }

    fn pick_next(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let next = self.current + 1;
        return self.source[next];
    }

    fn is_at_end(&self) -> bool {
        if self.current >= self.source.len() {
            return true;
        }
        return self.source[self.current] == '\0' || self.source[self.current] == '\n';
    }

    fn dot(&mut self, start: usize) -> Result<Token, &'static str> {
        if self.status == LexerStatus::IntNumber {
            self.status = LexerStatus::FloatNumber;
            self.number(start)
        } else {
            Err("Syntax error.")
        }
    }

    fn number(&mut self, start: usize) -> Result<Token, &'static str> {
        if self.status == LexerStatus::Initial {
            self.status = LexerStatus::IntNumber;
        }
        while self.peek().is_numeric() || self.peek() == '.' {
            if self.peek() == '.' {
                if self.status == LexerStatus::IntNumber {
                    self.status = LexerStatus::FloatNumber;
                } else {
                    return Err("Syntax error.");
                }
            }

            self.advance();
        }

        println!("current: {}, token_type: {:?}", self.current, self.status);
        let token_type = match self.status {
            LexerStatus::IntNumber | LexerStatus::FloatNumber => {
                TokenType::Number(self.source[start..self.current].iter().collect())
            }
            _ => return Err("Not a number."),
        };

        Ok(Token::new(token_type))
    }
}

#[cfg(test)]
mod tests {
    use crate::calculator::token::TokenType;

    use super::Lexer;

    #[test]
    fn base() {
        let mut lexer = Lexer::new("1");
        let token = lexer.get_token();
        assert!(token.is_ok());
        if let Ok(token) = token {
            assert_eq!(token.token_type, TokenType::Number("1".to_string()));
        }
        let token_eof = lexer.get_token();
        if let Ok(token) = token_eof {
            assert_eq!(token.token_type, TokenType::Eof);
        }
    }

    fn assert_token_eq(lexer: &mut Lexer, token_type: TokenType) {
        let token = lexer.get_token();
        if let Ok(token) = token {
            assert_eq!(token.token_type, token_type);
        }
    }

    #[test]
    fn base_opt() {
        let mut lexer = Lexer::new("1.23 + 1 * ( 2 + 3 ) - 4 / 2.5 + ( 6 / 2)");

        assert_token_eq(&mut lexer, TokenType::Number("1.23".to_string()));
        assert_token_eq(&mut lexer, TokenType::AddOperator);
        assert_token_eq(&mut lexer, TokenType::Number("1".to_string()));
        assert_token_eq(&mut lexer, TokenType::MulOperator);
        assert_token_eq(&mut lexer, TokenType::LeftParen);
        assert_token_eq(&mut lexer, TokenType::Number("2".to_string()));
        assert_token_eq(&mut lexer, TokenType::AddOperator);
        assert_token_eq(&mut lexer, TokenType::Number("3".to_string()));
        assert_token_eq(&mut lexer, TokenType::RightParen);
        assert_token_eq(&mut lexer, TokenType::SubOperator);
        assert_token_eq(&mut lexer, TokenType::Number("4".to_string()));
        assert_token_eq(&mut lexer, TokenType::DivOperator);
        assert_token_eq(&mut lexer, TokenType::Number("2.5".to_string()));
        assert_token_eq(&mut lexer, TokenType::AddOperator);
        assert_token_eq(&mut lexer, TokenType::LeftParen);
        assert_token_eq(&mut lexer, TokenType::Number("6".to_string()));
        assert_token_eq(&mut lexer, TokenType::DivOperator);
        assert_token_eq(&mut lexer, TokenType::Number("2".to_string()));
        assert_token_eq(&mut lexer, TokenType::RightParen);
        assert_token_eq(&mut lexer, TokenType::Eof);
    }
}
