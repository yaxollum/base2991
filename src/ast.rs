use std::fmt;

use super::num::Num;

#[derive(Debug)]
pub enum Ast {
    Literal(Num),
    BaseConv(Box<Ast>, u32),
    ReprAscii(Box<Ast>),
    Mul(Box<Ast>, Box<Ast>),
    Div(Box<Ast>, Box<Ast>),
    Add(Box<Ast>, Box<Ast>),
    Sub(Box<Ast>, Box<Ast>),
    NOP,
}

pub type AstResult = Result<Num, AstSpecial>;
pub enum AstSpecial {
    ReprAscii(Num),
    NOP,
}

impl fmt::Display for AstSpecial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Self::ReprAscii(n) = self {
            let n = n.as_u8();
            write!(
                f,
                "{} | {:04b} {:04b} | {:#x} | '{}'",
                n,
                n / 0x10,
                n % 0x10,
                n,
                n as char,
            )
        } else {
            Ok(())
        }
    }
}

impl Ast {
    pub fn eval(&self) -> AstResult {
        match self {
            Self::Literal(num) => Ok(num.clone()),
            Self::BaseConv(a, new_base) => Ok(a.eval()?.convert_base(*new_base)),
            Self::ReprAscii(a) => Err(AstSpecial::ReprAscii(a.eval()?)),
            Self::Mul(a, b) => Ok(a.eval()? * b.eval()?),
            Self::Div(a, b) => Ok(a.eval()? / b.eval()?),
            Self::Add(a, b) => Ok(a.eval()? + b.eval()?),
            Self::Sub(a, b) => Ok(a.eval()? - b.eval()?),
            Self::NOP => Err(AstSpecial::NOP),
        }
    }
}
