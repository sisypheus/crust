pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    LetStatement(Identifier, Expression),
    ReturnStatement(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier(pub String);
#[derive(Debug, PartialEq, Clone)]
pub struct Expression(pub String);
