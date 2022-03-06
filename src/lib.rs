mod ast;
mod num;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

pub use ast::AstResult;
pub use ast::AstSpecial;
use lalrpop_util::{lexer::Token, ParseError};

pub fn run(input: &str) -> Result<AstResult, ParseError<usize, Token, &str>> {
    Ok(parser::AstParser::new().parse(input)?.eval())
}
