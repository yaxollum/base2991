use base2991::{run, AstResult};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match run(&line.unwrap()) {
            Ok(ast_result) => match ast_result {
                AstResult::Value(val) => println!("{:?}", val),
                AstResult::NOP => {}
            },
            Err(e) => {
                eprintln!("SyntaxError: {}", e);
            }
        }
    }
}
