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
        self.skip_whitespace();

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
                ch => {
                    if ch.is_alphabetic() {
                        let literal = self.read_identifier();
                        token = new_token(is_identifier(literal.as_str()), literal);
                        return token;
                    } else if ch.is_digit(10) {
                        let literal = self.read_number();
                        token = new_token(TokenType::INT, literal);
                        return token;
                    } else {
                        token = new_token(TokenType::ILLEGAL, ch.to_string())
                    }
                }
            },
        };
        self.read_char();
        return token;
    }

    fn read_number(&mut self) -> String {
        let mut result = String::new();

        while self.ch.is_some() && self.ch.unwrap().is_digit(10) {
            result.push(self.ch.unwrap());
            self.read_char();
        }
        return result;
    }

    fn read_identifier(&mut self) -> String {
        let mut result = String::new();

        while self.ch.is_some() && self.ch.unwrap().is_alphabetic() {
            result.push(self.ch.unwrap());
            self.read_char();
        }
        return result;
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_some() && self.ch.unwrap().is_whitespace() {
            self.read_char();
        }
    }
}

fn new_token(token_type: TokenType, literal: String) -> Token {
    Token {
        token_type,
        literal,
    }
}

fn is_identifier(token: &str) -> TokenType {
    match token {
        "let" => TokenType::LET,
        "fn" => TokenType::FUNCTION,
        _ => TokenType::IDENT,
    }
}

#[cfg(test)]
mod tests {
    use crate::token;

    use super::Lexer;

    #[test]
    fn lexer_tokenizer_simple() {
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

    #[test]
    fn lexer_tokenizer_hard() {
        let input = "let five = 5;
let ten = 10;
   let add = fn(x, y) {
     x + y;
};
   let result = add(five, ten);";

        let expected = vec![
            token::Token {
                token_type: token::TokenType::LET,
                literal: "let".to_string(),
            },
            token::Token {
                token_type: token::TokenType::IDENT,
                literal: "five".to_string(),
            },
            token::Token {
                token_type: token::TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            token::Token {
                token_type: token::TokenType::INT,
                literal: "5".to_string(),
            },
            token::Token {
                token_type: token::TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            token::Token {
                token_type: token::TokenType::LET,
                literal: "let".to_string(),
            },
            token::Token {
                token_type: token::TokenType::IDENT,
                literal: "ten".to_string(),
            },
            token::Token {
                token_type: token::TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            token::Token {
                token_type: token::TokenType::INT,
                literal: "10".to_string(),
            },
            token::Token {
                token_type: token::TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            token::Token {
                token_type: token::TokenType::LET,
                literal: "let".to_string(),
            },
            token::Token {
                token_type: token::TokenType::IDENT,
                literal: "add".to_string(),
            },
            token::Token {
                token_type: token::TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            token::Token {
                token_type: token::TokenType::FUNCTION,
                literal: "fn".to_string(),
            },
            token::Token {
                token_type: token::TokenType::LPAREN,
                literal: "(".to_string(),
            },
            token::Token {
                token_type: token::TokenType::IDENT,
                literal: "x".to_string(),
            },
            token::Token {
                token_type: token::TokenType::COMMA,
                literal: ",".to_string(),
            },
            token::Token {
                token_type: token::TokenType::IDENT,
                literal: "y".to_string(),
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
                token_type: token::TokenType::IDENT,
                literal: "x".to_string(),
            },
            token::Token {
                token_type: token::TokenType::PLUS,
                literal: "+".to_string(),
            },
            token::Token {
                token_type: token::TokenType::IDENT,
                literal: "y".to_string(),
            },
            token::Token {
                token_type: token::TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            token::Token {
                token_type: token::TokenType::RBRACE,
                literal: "}".to_string(),
            },
            token::Token {
                token_type: token::TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
            token::Token {
                token_type: token::TokenType::LET,
                literal: "let".to_string(),
            },
            token::Token {
                token_type: token::TokenType::IDENT,
                literal: "result".to_string(),
            },
            token::Token {
                token_type: token::TokenType::ASSIGN,
                literal: "=".to_string(),
            },
            token::Token {
                token_type: token::TokenType::IDENT,
                literal: "add".to_string(),
            },
            token::Token {
                token_type: token::TokenType::LPAREN,
                literal: "(".to_string(),
            },
            token::Token {
                token_type: token::TokenType::IDENT,
                literal: "five".to_string(),
            },
            token::Token {
                token_type: token::TokenType::COMMA,
                literal: ",".to_string(),
            },
            token::Token {
                token_type: token::TokenType::IDENT,
                literal: "ten".to_string(),
            },
            token::Token {
                token_type: token::TokenType::RPAREN,
                literal: ")".to_string(),
            },
            token::Token {
                token_type: token::TokenType::SEMICOLON,
                literal: ";".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);

        for i in 0..expected.len() {
            let tok = lexer.next_token();
            println!("{:?}", tok);
            assert_eq!(tok.token_type, expected[i].token_type);
            assert_eq!(tok.literal, expected[i].literal);
        }
    }
}
