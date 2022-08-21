use crate::{
    ast::ast::{Expression, Identifier, Program, Statement, Statement::LetStatement},
    lexer::new_token,
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser {
    pub lexer: Lexer,
    pub current_token: Token,
    pub peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            current_token: new_token(TokenType::ILLEGAL, "".to_string()),
            peek_token: new_token(TokenType::ILLEGAL, "".to_string()),
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.current_token.token_type != TokenType::EOF {
            let statement = self.parse_statement();
            match statement {
                Some(stmt) => program.statements.push(stmt),
                None => (),
            }
            self.next_token();
        }

        program
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.token_type {
            TokenType::LET => {
                return self.parse_let_statement();
            }
            _ => {
                return None;
            }
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }

        let ident = Identifier(self.current_token.literal.to_string());

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        while !self.current_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(Statement::LetStatement(ident, Expression("".to_string())))
    }

    fn current_token_is(&self, t: TokenType) -> bool {
        self.current_token.token_type == t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.token_type == t
    }

    fn expect_peek(&mut self, token: TokenType) -> bool {
        if self.peek_token_is(token) {
            self.next_token();
            return true;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::ast::{Expression, Identifier, Statement},
        lexer::Lexer,
    };

    #[test]
    fn let_statements() {
        let input = "
let x = 5;
   let y = 10;
   let foobar = 838383;
";

        let lexer = Lexer::new(input);
        let mut parser = super::Parser::new(lexer);

        let program = parser.parse_program();

        if program.statements.len() != 3 {
            panic!("program.statements does not contain 3 statements");
        }

        let expected = vec![
            Statement::LetStatement(Identifier("x".to_string()), Expression("".to_string())),
            Statement::LetStatement(Identifier("y".to_string()), Expression("".to_string())),
            Statement::LetStatement(Identifier("foobar".to_string()), Expression("".to_string())),
        ];

        assert_eq!(program.statements, expected);
    }
}
