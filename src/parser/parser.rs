use crate::{
    ast::ast::Program,
    lexer::new_token,
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser {
    pub lexer: Lexer,
    pub current_token: TokenType,
    pub peek_token: TokenType,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            current_token: TokenType::ILLEGAL,
            peek_token: TokenType::ILLEGAL,
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token().token_type;
    }

    pub fn parse_program(&self) -> Program {
        Program { statements: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    #[test]
    fn let_statements() {
        let input = "
let x = 5;
   let y = 10;
   let foobar = 838383;
";

        let lexer = Lexer::new(input);
        let parser = super::Parser::new(lexer);

        let program = parser.parse_program();
        if program.statements.len() != 3 {
            panic!("program.statements does not contain 3 statements");
        }

        let expected = vec![
            "x",
            "y",
            "foobar",
        ];
    
    }
}
