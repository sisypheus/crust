#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    MINUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
    FSLASH,
    BANG,
    STAR,
    LESS,
    GREATER,
    IF,
    ELSE,
    RETURN,
    TRUE,
    FALSE,
    EQ,
    NOT_EQ,
}
