#[derive(Debug)]
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
        "true" => TokenType::True,
        "false" => TokenType::False,
        _ => TokenType::Identifier,
    }
}

impl Token {
    pub fn new(token_type: TokenType, literal: String, line: usize, column: usize) -> Self {
        Token {
            token_type,
            literal,
            line,
            column,
        }
    }
}
