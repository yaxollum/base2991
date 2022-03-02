#[derive(Debug)]
enum Base {
    Bin,
    Oct,
    Hex,
    Dec,
}

#[derive(Debug)]
pub struct Num {
    base: Base,
    precision: u64,
    val: f64,
}

impl Num {
    pub fn from_str(s: &str) -> Self {
        Self {
            base: Base::Hex,
            precision: 69,
            val: 420.0,
        }
    }
}
