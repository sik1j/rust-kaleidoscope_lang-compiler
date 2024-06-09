mod parser;
mod lexer;

use std::io::Write;
use parser::Parser;

fn main() {
    print!("ready> ");
    std::io::stdout().flush().unwrap();
    let mut p = Parser::new();
    p.main_loop();
}
