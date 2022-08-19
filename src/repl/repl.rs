use std::io::{self, Write};

use crate::lexer::Lexer;

pub fn start() {
    let mut input = String::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let mut lexer = Lexer::new(&input);
        loop {
            let token = lexer.next_token();
            if token.token_type == crate::token::TokenType::EOF {
                break;
            }
            println!("{:?}", token);
        }
        input.clear();
    }
}
