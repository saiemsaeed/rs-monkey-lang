use crate::token::{Token, TokenType};

pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub read_position: usize,
    pub line: usize,
    pub column: usize,
    pub current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            line: 1,
            column: 1,
            current_char: None,
        };

        lexer.read_char();
        return lexer;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let peek_char = self.peek_char();

        let token: Token = match self.current_char {
            Some('=') => {
                if peek_char == Some('=') {
                    self.read_char();
                    self.new_token(TokenType::Equals, "==")
                } else {
                    self.new_token(TokenType::Assign, "=")
                }
            }
            Some('+') => self.new_token(TokenType::Plus, "+"),
            Some('-') => self.new_token(TokenType::Minus, "-"),
            Some('*') => self.new_token(TokenType::Asterisk, "*"),
            Some('/') => self.new_token(TokenType::Slash, "/"),
            Some('!') => {
                if peek_char == Some('=') {
                    self.read_char();
                    self.new_token(TokenType::NotEquals, "!=")
                } else {
                    self.new_token(TokenType::Bang, "!")
                }
            }
            Some('<') => self.new_token(TokenType::LT, "<"),
            Some('>') => self.new_token(TokenType::GT, ">"),
            Some(',') => self.new_token(TokenType::Comma, ","),
            Some(';') => self.new_token(TokenType::Semicolon, ";"),
            Some('(') => self.new_token(TokenType::LeftParen, "("),
            Some(')') => self.new_token(TokenType::RightParen, ")"),
            Some('{') => self.new_token(TokenType::LeftBrace, "{"),
            Some('}') => self.new_token(TokenType::RightBrace, "}"),
            Some('0'..= '9') => self.read_number(),  
            Some('a'..= 'z') | Some('A'..= 'Z') | Some('_') => {
                self.read_identifier()
            }
            None => self.new_token(TokenType::EOF, ""),
            _ => self.new_token(TokenType::Illegal, &self.current_char.unwrap_or('?').to_string()),
        };

        self.read_char();
        return token;
    }

    pub fn read_identifier(&mut self) -> Token {
        let start_position = self.position;
        while let Some(c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' {
                self.read_char();
            } else {
                break;
            }
        }
        let literal = &self.input[start_position..=self.position];
        let token_type = crate::token::lookup_ident(literal);
        return self.new_token(token_type, literal);
    }

    pub fn read_number(&mut self) -> Token {
        let start_position = self.position;
        while let Some(c) = self.peek_char() {
            if c.is_digit(10) {
                self.read_char();
            } else {
                break;
            }
        }
        let literal = &self.input[start_position..=self.position];
        let value: u64 = literal.parse().unwrap_or(0);
        return self.new_token(TokenType::Integer(value), literal);
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.current_char = None;
        } else {
            self.current_char = self.input.chars().nth(self.read_position);
        }
        self.position = self.read_position;
        self.read_position += 1;

        if self.current_char == Some('\n') {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
    }

    pub fn peek_char(&self) -> Option<char> {
        if self.read_position >= self.input.len() {
            return None;
        }
        return self.input.chars().nth(self.read_position);
    }

    fn new_token(&self, token_type: TokenType, literal: &str) -> Token {
        return Token {
            token_type,
            literal: literal.to_string(),
            line: self.line,
            column: self.column - literal.len(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
"#;

        let tests = vec![
            (TokenType::Let, "let", 1, 1),
            (TokenType::Identifier("five".to_string()), "five", 1, 5),
            (TokenType::Assign, "=", 1, 10),
            (TokenType::Integer(5), "5", 1, 12),
            (TokenType::Semicolon, ";", 1, 13),
            (TokenType::Let, "let", 2, 1),
            (TokenType::Identifier("ten".to_string()), "ten", 2, 5),
            (TokenType::Assign, "=", 2, 9),
            (TokenType::Integer(10), "10", 2, 11),
            (TokenType::Semicolon, ";", 2, 13),
            (TokenType::Let, "let", 4, 1),
            (TokenType::Identifier("add".to_string()), "add", 4, 5),
            (TokenType::Assign, "=", 4, 9),
            (TokenType::Function, "fn", 4, 11),
            (TokenType::LeftParen, "(", 4, 13),
            (TokenType::Identifier("x".to_string()), "x", 4, 14),
            (TokenType::Comma, ",", 4, 15),
            (TokenType::Identifier("y".to_string()), "y", 4, 17),
            (TokenType::RightParen, ")", 4, 18),
            (TokenType::LeftBrace, "{", 4, 20),
            (TokenType::Identifier("x".to_string()), "x", 5, 3),
            (TokenType::Plus, "+", 5, 5),
            (TokenType::Identifier("y".to_string()), "y", 5, 7),
            (TokenType::Semicolon, ";", 5, 8),
            (TokenType::RightBrace, "}", 6, 1),
            (TokenType::Semicolon, ";", 6, 2),
            (TokenType::Let, "let", 8, 1),
            (TokenType::Identifier("result".to_string()), "result", 8, 5),
            (TokenType::Assign, "=", 8, 12),
            (TokenType::Identifier("add".to_string()), "add", 8, 14),
            (TokenType::LeftParen, "(", 8, 17),
            (TokenType::Identifier("five".to_string()), "five", 8, 18),
            (TokenType::Comma, ",", 8, 22),
            (TokenType::Identifier("ten".to_string()), "ten", 8, 24),
            (TokenType::RightParen, ")", 8, 27),
            (TokenType::Semicolon, ";", 8, 28),
            (TokenType::Bang, "!", 9, 1),
            (TokenType::Minus, "-", 9, 2),
            (TokenType::Slash, "/", 9, 3),
            (TokenType::Asterisk, "*", 9, 4),
            (TokenType::Integer(5), "5", 9, 5),
            (TokenType::Semicolon, ";", 9, 6),
            (TokenType::Integer(5), "5", 10, 1),
            (TokenType::LT, "<", 10, 3),
            (TokenType::Integer(10), "10", 10, 5),
            (TokenType::GT, ">", 10, 8),
            (TokenType::Integer(5), "5", 10, 10),
            (TokenType::Semicolon, ";", 10, 11),
            (TokenType::If, "if", 12, 1),
            (TokenType::LeftParen, "(", 12, 4),
            (TokenType::Integer(5), "5", 12, 5),
            (TokenType::LT, "<", 12, 7),
            (TokenType::Integer(10), "10", 12, 9),
            (TokenType::RightParen, ")", 12, 11),
            (TokenType::LeftBrace, "{", 12, 13),
            (TokenType::Return, "return", 13, 5),
            (TokenType::Bool(true), "true", 13, 12),
            (TokenType::Semicolon, ";", 13, 16),
            (TokenType::RightBrace, "}", 14, 1),
            (TokenType::Else, "else", 14, 3),
            (TokenType::LeftBrace, "{", 14, 8),
            (TokenType::Return, "return", 15, 5),
            (TokenType::Bool(false), "false", 15, 12),
            (TokenType::Semicolon, ";", 15, 17),
            (TokenType::RightBrace, "}", 16, 1),
            (TokenType::Integer(10), "10", 18, 1),
            (TokenType::Equals, "==", 18, 4),
            (TokenType::Integer(10), "10", 18, 7),
            (TokenType::Semicolon, ";", 18, 9),
            (TokenType::Integer(10), "10", 19, 1),
            (TokenType::NotEquals, "!=", 19, 4),
            (TokenType::Integer(9), "9", 19, 7),
            (TokenType::Semicolon, ";", 19, 8),
        ];

        let mut lexer = Lexer::new(input.to_string());

        for (i, (expected_type, expected_literal, expected_line, expected_column)) in tests.iter().enumerate() {
            let token = lexer.next_token();
            
            if i == 73 {
                println!("Test 73: token={:?}, literal={}, line={}, column={}", token.token_type, token.literal, token.line, token.column);
                println!("Expected: type={:?}, literal={}, line={}, column={}", expected_type, expected_literal, expected_line, expected_column);
            }

            assert_eq!(token.token_type, *expected_type, "tests[{}] - tokentype wrong. expected={:?}, got={:?}", i, expected_type, token.token_type);
            assert_eq!(token.literal, *expected_literal, "tests[{}] - literal wrong. expected={:?}, got={:?}", i, expected_literal, token.literal);
            assert_eq!(token.line, *expected_line, "tests[{}] - line wrong. expected={}, got={}", i, expected_line, token.line);
            assert_eq!(token.column, *expected_column, "tests[{}] - column wrong. expected={}, got={}", i, expected_column, token.column);
        }
    }
}
