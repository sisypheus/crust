struct Lexer {
    pub input: String,
    pub position: usize,
    pub read_position: usize,
    pub ch: Option<char>,
}

impl Lexer {
    fn new(input: &str) -> Lexer {
        let l = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: None,
        };
        l.read_char();
        return l
    }

    fn read_char(&self) {
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
            token::TokenType {
                token_type: token::Tokens::ASSIGN,
                literal: "=".to_string(),
            },
            token::TokenType {
                token_type: token::Tokens::PLUS,
                literal: "+".to_string(),
            },
            token::TokenType {
                token_type: token::Tokens::LPAREN,
                literal: "(".to_string(),
            },
            token::TokenType {
                token_type: token::Tokens::RPAREN,
                literal: ")".to_string(),
            },
            token::TokenType {
                token_type: token::Tokens::LBRACE,
                literal: "{".to_string(),
            },
            token::TokenType {
                token_type: token::Tokens::RBRACE,
                literal: "}".to_string(),
            },
            token::TokenType {
                token_type: token::Tokens::COMMA,
                literal: ",".to_string(),
            },
            token::TokenType {
                token_type: token::Tokens::SEMICOLON,
                literal: ";".to_string(),
            },
        ];

        let lexer = Lexer::new(input);
    }
}
