use tokenizer::Token;
struct Condition {
    comparator: Token,
}

impl Condition {
    pub fn new(conditional_token: Token) -> Condition {
        match conditional_token {
            Token::Less
            | Token::LessEqual
            | Token::Equal
            | Token::GreaterEqual
            | Token::Greater => Condition {
                comparator: conditional_token,
            },
            _ => panic!("Attempting to instatiate Condition with {}", comparator),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_tokens() {
        let valid_tokens = [
            Token::Less,
            Token::LessEqual,
            Token::Equal,
            Token::GreaterEqual,
            Token::Greater,
        ];
        for token in valid_tokens.iter().cloned() {
            let cond = Condition::new(token.clone());
            assert_eq!(cond.comparator, token);
        }
    }

    #[test]
    #[should_panic(expected = "Attempting to instantiate Condition with invalid token")]
    fn test_invalid_token() {
        Condition::new(Token::Plus); // should panic
    }
}
