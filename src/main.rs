mod token;
mod lexer;
mod repl;
mod ast;
mod parser;

fn main() {
    println!("Welcome to the crust programming language!");
    repl::start();
}
