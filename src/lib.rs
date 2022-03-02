mod ast;
mod num;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

use num::Num;

pub enum RunResult {
    Value(Num),
    ParseError,
}

pub fn run(input: &str) -> RunResult {
    if let Ok(ast) = parser::AstParser::new().parse(input) {
        RunResult::Value(ast.eval())
    } else {
        RunResult::ParseError
    }
}
