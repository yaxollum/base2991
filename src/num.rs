use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Num {
    base: u32,
    val: f64,
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

impl Div for Num {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self {
            base: self.base,
            val: self.val / rhs.val,
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
        let val = if let Some((a_str, b_str)) = s.split_once('.') {
            let b_str = &b_str[..b_str.len() - 1];
            let a = u64::from_str_radix(a_str, base).ok()? as f64;
            let b = u64::from_str_radix(b_str, base).ok()? as f64;
            a + 1.0 / (base as f64).powi(b_str.len() as i32) * b
        } else {
            u64::from_str_radix(&s[..s.len() - 1], base).ok()? as f64
        };
        Some(Self { base, val })
    }
    pub fn convert_base(&self, new_base: u32) -> Self {
        Self {
            base: new_base,
            val: self.val,
        }
    }
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const PRECISION: i32 = 10;
        let base = self.base as i64;
        let mut val_dec = self.val.trunc() as i64;
        let mut val_frac = (self.val.fract() * (self.base as f64).powi(PRECISION)).round() as i64;
        let digits = b"0123456789abcdef";
        let mut dec_chars = Vec::new();
        if val_dec == 0 {
            dec_chars.push(b'0');
        }
        while val_dec != 0 {
            dec_chars.push(digits[(val_dec % base) as usize]);
            val_dec /= base;
        }
        let mut frac_chars = Vec::new();
        while val_frac != 0 {
            frac_chars.push(digits[(val_frac % base) as usize]);
            val_frac /= base;
        }

        for c in dec_chars.iter().rev() {
            write!(f, "{}", *c as char)?;
        }
        write!(f, ".")?;

        while frac_chars.len() < PRECISION as usize {
            frac_chars.push(b'0');
        }
        for c in frac_chars.iter().rev() {
            write!(f, "{}", *c as char)?;
        }
        write!(f, "_{}", base)
    }
}
