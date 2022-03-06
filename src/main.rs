use base2991::run;
use base2991::AstSpecial;
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
                Err(special) => match special {
                    AstSpecial::NOP => {}
                    other => println!("{}", other),
                },
            },
            Err(e) => {
                eprintln!("SyntaxError: {}", e);
            }
        }
        prompt();
    }
}
