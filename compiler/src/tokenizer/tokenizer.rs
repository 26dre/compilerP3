use crate::tokenizer::tokens::Token;
pub struct Tokenizer {
    input: Vec<u8>,
    pos: usize,
}

impl Tokenizer {
    /// Create a new tokenizer with the provided input string.
    pub fn new(input: String) -> Self {
        Self {
            input: input.into_bytes(),
            pos: 0,
        }
    }

    /// Peek at the current character without advancing the tokenizer.
    pub fn peek_char(&self) -> char {
        if self.pos >= self.input.len() {
            '\0'
        } else {
            self.input[self.pos] as char
        }
    }

    /// Advance the tokenizer and return the next character.
    fn next_char(&mut self) -> char {
        let c = self.peek_char();
        self.pos += 1;
        c
    }

    /// Consume characters while the provided function returns true.
    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while self.pos < self.input.len() && test(self.peek_char()) {
            result.push(self.next_char());
        }
        result
    }

    /// Consume all whitespace characters from the current position.
    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    /// Tokenize a sequence of digits into a Number token.
    fn tokenize_number(&mut self) -> Token {
        let number_str = self.consume_while(|c| c.is_digit(10));
        Token::Number(number_str.parse::<isize>().unwrap())
    }

    /// Tokenize an identifier or a keyword into the appropriate Token type.
    fn tokenize_identifier_or_keyword(&mut self) -> Token {
        let identifier = self.consume_while(|c| c.is_alphanumeric() || c == '_');
        match identifier.as_str() {
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "function" => Token::Function,
            "call" => Token::FunctionCall,
            "return" => Token::Return,
            "var" => Token::Variable,
            "void" => Token::Void,
            "then" => Token::Then,
            "fi" => Token::Fi,
            "do" => Token::Do,
            "od" => Token::Od,
            "let" => Token::Let,
            "main" => Token::Main,
            _ => Token::Identifier(identifier),
        }
    }

    /// Tokenize a relational operator or the assignment operator.
    fn tokenize_operator(&mut self) -> Token {
        let op = self.next_char();
        match op {
            '<' => {
                if self.peek_char() == '-' {
                    self.next_char(); // Consume '-'
                    Token::Assignment
                } else if self.peek_char() == '=' {
                    self.next_char(); // Consume '='
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.next_char(); // Consume '='
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
            '=' => {
                if self.peek_char() == '=' {
                    self.next_char(); // Consume '='
                    Token::Equal
                } else {
                    panic!("Unexpected character after '=': {}", self.peek_char());
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.next_char(); // Consume '='
                    Token::NotEqual
                } else {
                    panic!("Unexpected character after '!': {}", self.peek_char());
                }
            }
            _ => panic!("Unexpected operator: {}", op),
        }
    }

    /// Peeks the next token from the input, without advancing the tokenizer.
    pub fn peek_token(&mut self) -> Token {
        let previous_pos = self.pos;
        let token = self.next_token();
        self.pos = previous_pos;

        token
    }

    /// Retrieve the next token from the input, advancing the tokenizer.
    pub fn next_token(&mut self) -> Token {
        self.consume_whitespace();

        let token = match self.peek_char() {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Times,
            '/' => Token::Divide,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            '.' => Token::EOF,
            '0'..='9' => return self.tokenize_number(),
            '<' | '>' | '=' | '!' => return self.tokenize_operator(),
            'a'..='z' | 'A'..='Z' => return self.tokenize_identifier_or_keyword(),
            _ => panic!("Unexpected character: {}", self.peek_char()),
        };

        self.next_char();
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_test() {
        assert_eq!(10, 10)
    }
}
