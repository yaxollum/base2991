use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Num {
    base: u32,
    val: u64,
}

impl Mul for Num {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            base: self.base,
            val: self.val * rhs.val,
        }
    }
}

impl Add for Num {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            base: self.base,
            val: self.val + rhs.val,
        }
    }
}

impl Sub for Num {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            base: self.base,
            val: self.val - rhs.val,
        }
    }
}

impl Num {
    pub fn from_str(s: &str) -> Option<Self> {
        let base = match s.chars().last()? {
            'x' | 'h' => Some(16),
            'd' => Some(10),
            'o' => Some(8),
            'b' => Some(2),
            _ => None,
        }?;
        Some(Self {
            base,
            val: u64::from_str_radix(&s[..s.len() - 1], base).ok()?,
        })
    }
    pub fn convert_base(&self, new_base: u32) -> Self {
        Self {
            base: new_base,
            val: self.val,
        }
    }
}
