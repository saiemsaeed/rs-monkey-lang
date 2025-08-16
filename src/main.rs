mod lexer;
mod token;

use lexer::Lexer;

fn main() {
    // let input = "fn sum(a, b) { a + b }".to_string();
    let input = "=+=".to_string();
    let mut lexer = Lexer::new(input);
    println!("Reading all characters:");
    while let Some(ch) = lexer.current_char {
        println!("Character: '{}'", ch);
        let token = lexer.next_token();
        println!("Token: {:?}", token.token_type);

    }
    println!("Done reading all characters.");
}
