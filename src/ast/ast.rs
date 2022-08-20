use crate::token;

struct LetStatement {
    token: token::Token,
    name: Identifier,
    value: Expression,
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
    }
}

pub struct Expression {
    pub token_literal: String,
}

pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

impl Identifier {
    pub fn expression_node(&self) {}
    pub fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }
}

pub struct Statement {
    pub token_literal: String,
}

impl Statement {
    pub fn statement_node(&self) {}
}

pub trait Node {
    fn token_literal(&self) -> String;
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        self.token_literal.to_string()
    }
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        self.token_literal.to_string()
    }
}

impl LetStatement {
    pub fn token_literals(&self) -> String {
        self.token.literal.to_string()
    }
}
