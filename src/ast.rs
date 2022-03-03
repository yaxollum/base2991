use super::num::Num;

#[derive(Debug)]
pub enum Ast {
    Literal(Num),
    BaseConv(Box<Ast>, u32),
    Mul(Box<Ast>, Box<Ast>),
    Add(Box<Ast>, Box<Ast>),
    NOP,
}

pub type AstResult = Result<Num, AstError>;
pub enum AstError {
    NOP,
}

impl Ast {
    pub fn eval(&self) -> AstResult {
        match self {
            Self::Literal(num) => Ok(num.clone()),
            Self::BaseConv(a, new_base) => Ok(a.eval()?.convert_base(*new_base)),
            Self::Mul(a, b) => Ok(a.eval()? * b.eval()?),
            Self::Add(a, b) => Ok(a.eval()? + b.eval()?),
            Self::NOP => Err(AstError::NOP),
        }
    }
}
