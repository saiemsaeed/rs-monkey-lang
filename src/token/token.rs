
#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Operators 
    Equals, 
    NotEquals,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LT,
    GT,

    // Delimiters
    Comma,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // Literal and Identifiers
    Identifier(String),
    Integer(u64),
    Bool(bool),

    // Reserved Keywords
    Function,
    Let,
    Return,
    Assign,
    If,
    Else,

    // Special Tokens
    Illegal, 
    EOF
}


pub struct Token {
    pub token_type:TokenType,
    pub literal: String,
    pub line: usize,
    pub column: usize,
}


pub fn lookup_ident(ident: &str) -> TokenType {
   return match ident {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        "true" => TokenType::Bool(true),
        "false" => TokenType::Bool(false),
        _ => TokenType::Identifier(ident.to_string()),
    }
}

