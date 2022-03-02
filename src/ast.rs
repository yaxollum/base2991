use super::num::Num;

#[derive(Debug)]
pub enum Ast {
    Literal(Num),
    NOP,
}

impl Ast {
    pub fn eval(&self) -> Num {
        Num::from_str("hello")
    }
}
