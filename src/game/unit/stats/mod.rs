pub mod core
pub mod skills

#[derive(Debug, Clone, Copy)]
pub enum Shade {
    Black,
    Grey,
    White,
}

#[derive(Debug, Clone, Copy)]
pub struct Stat {
    exponent: f64,
    shade: Shade,
    routine: f64,
    difficult: f64,
    challenging: f64,
}

impl Stat {
    fn new() -> Self {
        
    }
}
