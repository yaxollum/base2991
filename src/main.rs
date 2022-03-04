use base2991::run;
use std::io::{self, BufRead, Write};

fn prompt() {
    print!("> ");
    std::io::stdout().flush().unwrap();
}

fn main() {
    let stdin = io::stdin();

    prompt();
    for line in stdin.lock().lines() {
        match run(&line.unwrap()) {
            Ok(ast_result) => match ast_result {
                Ok(val) => println!("{}", val),
                Err(_) => {}
            },
            Err(e) => {
                eprintln!("SyntaxError: {}", e);
            }
        }
        prompt();
    }
}
