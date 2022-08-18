pub struct TokenType {
    pub token_type: Tokens,
    pub literal: String,
}

pub enum Tokens {
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
}
// type TokenList = Vec<Token>;
