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
        return lexer
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

}
