#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

#[derive(Debug, PartialEq)]
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
}
// type TokenList = Vec<Token>;
