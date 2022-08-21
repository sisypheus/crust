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
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            current_token: new_token(TokenType::ILLEGAL, "".to_string()),
            peek_token: new_token(TokenType::ILLEGAL, "".to_string()),
            errors: Vec::new(),
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
            TokenType::RETURN => {
                return self.parse_return_statement();
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

    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        while !self.current_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }

        Some(Statement::ReturnStatement(Expression("".to_string())))
    }

    fn current_token_is(&self, t: TokenType) -> bool {
        self.current_token.token_type == t
    }

    fn peek_token_is(&self, t: &TokenType) -> bool {
        self.peek_token.token_type == *t
    }

    fn expect_peek(&mut self, token: TokenType) -> bool {
        if self.peek_token_is(&token) {
            self.next_token();
            return true;
        } else {
            self.peek_error(&token);
            return false;
        }
    }

    fn peek_error(&mut self, t: &TokenType) {
        let token_literal = self.peek_token.literal.clone();
        self.errors.push(format!(
            "expected {:?}, but got {} instead",
            t, token_literal
        ));
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

    #[test]
    fn parse_error() {
        let input = "
let x 5;
let y 10;
let foobar 838383;
";
            
            let lexer = Lexer::new(input);
            let mut parser = super::Parser::new(lexer);
    
            parser.parse_program();
    
            assert!(parser.errors.len() == 3);
    }

    #[test]
    fn return_statements() {
        let input = "
return 5;
return 10;
return (add(5, 10));
";

        let lexer = Lexer::new(input);
        let mut parser = super::Parser::new(lexer);

        let program = parser.parse_program();

        if program.statements.len() != 3 {
            panic!("program.statements does not contain 3 statements");
        }

        if parser.errors.len() != 0 {
            panic!("parser has {} errors", parser.errors.len());
        }
        let expected = vec![
            Statement::ReturnStatement(Expression("".to_string())),
            Statement::ReturnStatement(Expression("".to_string())),
            Statement::ReturnStatement(Expression("".to_string())),
        ];

        assert_eq!(program.statements, expected);
    }
}
