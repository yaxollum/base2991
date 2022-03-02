use std::num::ParseIntError;

#[derive(Debug, Clone, Copy)]
enum Base {
    Bin,
    Oct,
    Hex,
    Dec,
}

#[derive(Debug, Clone)]
pub struct Num {
    base: Base,
    val: i64,
}

impl Num {
    pub fn from_str(s: &str) -> Result<Self, ParseIntError> {
        Ok(Self {
            base: Base::Hex,
            val: s[..s.len() - 1].parse()?,
        })
    }
}
