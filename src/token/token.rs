pub struct Token {
    token_type: String,
    literal: String,
    line: usize,
    column: usize,
}

pub enum TokenType {
    Equals, 
    NotEquals,
    Plus,
    Minus,
    Multiply,
    Divide,
    Bang,
    Asterisk,
    Slash,
    LT,
    GT,
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Identifier,
    Integer,
    String,
    True,
    False,
    Assign,
    Illegal, 
    Function,
    If,
    Else,
    Return,
    Let,
}

pub fn lookup_ident(ident: &str) -> TokenType {
   return match ident {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        "true" => TokenType::True,
        "false" => TokenType::False,
        _ => TokenType::Identifier,
    }
}

impl Token {
    pub fn new(token_type: String, literal: String, line: usize, column: usize) -> Self {
        Token {
            token_type,
            literal,
            line,
            column,
        }
    }
}
