use crate::token::{Token, TokenType};

struct Lexer {
    pub input: String,
    pub position: usize,
    pub read_position: usize,
    pub ch: Option<char>,
}

impl Lexer {
    fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: None,
        };
        l.read_char();
        return l;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input.chars().nth(self.read_position).unwrap());
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let token: Token;

        match self.ch {
            None => token = new_token(TokenType::ILLEGAL, "".to_string()),
            Some(ch) => match ch {
                '=' => token = new_token(TokenType::ASSIGN, ch.to_string()),
                ';' => token = new_token(TokenType::SEMICOLON, ch.to_string()),
                '(' => token = new_token(TokenType::LPAREN, ch.to_string()),
                ')' => token = new_token(TokenType::RPAREN, ch.to_string()),
                '{' => token = new_token(TokenType::LBRACE, ch.to_string()),
                '}' => token = new_token(TokenType::RBRACE, ch.to_string()),
                '+' => token = new_token(TokenType::PLUS, ch.to_string()),
                '-' => token = new_token(TokenType::MINUS, ch.to_string()),
                ',' => token = new_token(TokenType::COMMA, ch.to_string()),
                _ => token = new_token(TokenType::ILLEGAL, ch.to_string()),
            },
        };
        self.read_char();
        return token;
    }
}

fn new_token(token_type: TokenType, literal: String) -> Token {
    Token {
        token_type,
        literal,
    }
}

#[cfg(test)]
mod tests {
    use crate::token;

    use super::Lexer;

    #[test]
    fn lexer_tokenizer() {
        let input = "=+(){},;";

        let expected = vec![
            token::Token {
                token_type: token::TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            token::Token {
                token_type: token::TokenType::PLUS,
                literal: "+".to_string(),
            },
            token::Token {
                token_type: token::TokenType::LPAREN,
                literal: "(".to_string(),
            },
            token::Token {
                token_type: token::TokenType::RPAREN,
                literal: ")".to_string(),
            },
            token::Token {
                token_type: token::TokenType::LBRACE,
                literal: "{".to_string(),
            },
            token::Token {
                token_type: token::TokenType::RBRACE,
                literal: "}".to_string(),
            },
            token::Token {
                token_type: token::TokenType::COMMA,
                literal: ",".to_string(),
            },
            token::Token {
                token_type: token::TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);

        for i in 0..expected.len() {
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, expected[i].token_type);
            assert_eq!(tok.literal, expected[i].literal);
        }
    }
}
