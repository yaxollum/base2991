use super::num::Num;

#[derive(Debug)]
pub enum Ast {
    Literal(Num),
    NOP,
}

pub enum AstResult {
    Value(Num),
    NOP,
}

impl Ast {
    pub fn eval(&self) -> AstResult {
        match self {
            Self::Literal(num) => AstResult::Value(num.clone()),
            Self::NOP => AstResult::NOP,
        }
    }
}
