use super::{
    lexer::{self, Lexer},
    token::{Token, TokenType},
};

/**
 Syntax
 expression  : term
             | expression + term
             | expression - term

 term        : primary_expr
             | primary_expr * term
             | primary_expr / term

 primary_expr: double_value
             | (expression)

*/
pub struct Parser {
    lexer: Lexer,
    look_ahead_token: Option<Token>,
}

impl Parser {
    pub fn new(source: &str) -> Self {
        Parser {
            lexer: Lexer::new(source),
            look_ahead_token: None,
        }
    }

    pub fn parse(&mut self) -> f64 {
        return self.expression();
    }

    fn get_token(&mut self) -> Token {
        if let Some(token) = self.look_ahead_token.clone() {
            self.look_ahead_token = None;
            return token;
        }
        self.lexer.get_token().expect("Get token error.")
    }

    fn back_token(&mut self, token: Token) {
        self.look_ahead_token = Some(token);
    }

    fn expression(&mut self) -> f64 {
        let mut left_number = 0.0;
        let mut right_number = 0.0;

        left_number = self.term();

        loop {
            let token = self.get_token();

            if token.token_type != TokenType::AddOperator
                && token.token_type != TokenType::SubOperator
            {
                self.back_token(token);
                break;
            }

            right_number = self.term();

            if token.token_type == TokenType::AddOperator {
                left_number += right_number;
            } else if token.token_type == TokenType::SubOperator {
                left_number -= right_number;
            } else {
                self.back_token(token);
            }
        }

        left_number
    }

    fn term(&mut self) -> f64 {
        let mut left_number = 0.0;
        let mut right_number = 0.0;

        left_number = self.primary_expr();

        loop {
            let token = self.get_token();
            if token.token_type != TokenType::MulOperator
                && token.token_type != TokenType::DivOperator
            {
                self.back_token(token);
                break;
            }

            right_number = self.primary_expr();
            if token.token_type == TokenType::MulOperator {
                left_number *= right_number;
            } else if token.token_type == TokenType::DivOperator {
                left_number /= right_number;
            }
        }
        left_number
    }

    fn primary_expr(&mut self) -> f64 {
        let mut has_minus_flag = false;
        let mut number: f64 = 0.0;

        // 检查下是否存在负数标识符
        let mut token = self.get_token();
        if token.token_type == TokenType::SubOperator {
            has_minus_flag = true;
        } else {
            self.back_token(token);
        }

        token = self.get_token();
        match token.token_type {
            TokenType::Number(value) => number = value.parse().expect("Failed to parse number"),
            TokenType::LeftParen => {
                number = self.expression();
                token = self.get_token();
                if token.token_type != TokenType::RightParen {
                    panic!("Missing ')' error.");
                }
            }
            _ => {
                self.back_token(token);
            }
        }
        if has_minus_flag {
            number = -number;
        }
        number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        let mut parser = Parser::new("1+2+3");
        let result = parser.parse();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn base_2() {
        let mut parser = Parser::new("1 + 2 * 3 + 4");
        assert_eq!(parser.parse(), 11.0);
    }

    #[test]
    fn base_3() {
        let mut parser = Parser::new("1 + 2 * (2 * (3 + 4)) + 6 / 3");
        assert_eq!(parser.parse(), 31.0);
    }

    #[test]
    fn base_4() {
        let mut parser = Parser::new("0.0123 + 4 * 0.25 - 3 / 3.5");
        assert_eq!((parser.parse() * 1000.0).round() / 1000.0, 0.155);
    }
}
