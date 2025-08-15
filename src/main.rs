mod lexer;
mod token;

use lexer::Lexer;

fn main() {
    let input = "fn sum(a, b) { a + b }".to_string();
    let mut lexer = Lexer::new(input);
    println!("Reading all characters:");
    while let Some(ch) = lexer.current_char {
        println!("Character: '{}'", ch);
        lexer.read_char();
    }
    println!("Done reading all characters.");
}
