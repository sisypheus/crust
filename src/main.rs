mod token;
mod lexer;
mod repl;

fn main() {
    println!("Welcome to the Monkey programming language!");
    repl::start();
}
