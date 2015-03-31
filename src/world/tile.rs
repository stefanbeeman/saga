use std::clone::Clone;

#[derive(Clone)]
pub enum Floor {
    Grass(u32),
}

#[derive(Clone)]
pub struct Tile {
    pub floor: Floor,
}
