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
        let token;

        match self.current_char {
            Some('=') => {
                token = self.new_token(TokenType::Equals, '='.to_string());
            },
            Some('+') => {
                token = self.new_token(TokenType::Plus, '+'.to_string())

            }
            _ => {
                token = self.new_token(TokenType::EOF, '\0'.to_string())
            }
        }

        self.read_char();
        return token
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

    fn new_token(&self, token_type: TokenType, literal: String) -> Token {
        return Token {
            token_type,
            literal,
            line: 1,
            column: 1,
        };
    }
}
