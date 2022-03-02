use base2991::{run, RunResult};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match run(&line.unwrap()) {
            RunResult::ParseError => {
                eprintln!("Could not parse expression");
            }
            RunResult::Value(num) => {
                println!("{:?}", num);
            }
        }
    }
}
