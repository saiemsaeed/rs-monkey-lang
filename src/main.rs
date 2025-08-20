mod lexer;
mod token;

use lexer::Lexer;

use crate::token::TokenType;

fn main() {
    // let input = "fn sum(a, b) { a + b }".to_string();
    let input = "let five = 5;
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
10 != 9;".to_string();
    let mut lexer = Lexer::new(input);
    loop {
        let token = lexer.next_token();
        match token.token_type {
            TokenType::EOF => {
                println!("Done reading all characters.");
                break;
            }
            TokenType::Illegal => {
                println!("Illegal token encountered: {}", token.literal);
                continue;
            }
            _ => {
                println!("Token: {:?}, Literal {:?} Line {:?} Column {:?}", token.token_type, token.literal, token.line, token.column);
            }
        }
    }
}
