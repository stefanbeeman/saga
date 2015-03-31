use na::{ Pnt2, Vec2 };
use nd::{ Array, arr2, Ix };
use world::tile::{ Floor, Tile };

#[derive(Clone)]
pub struct Layer {
    tiles: Array<Tile, (Ix, Ix)>,
}

impl Layer {
    pub fn new() -> Self {
        Layer {
            tiles: Array::from_elem((10, 10), Tile {
                floor: Floor::Grass(0),
            }),
        }
    }
    pub fn at(&self, pos: Pnt2<u32>) -> Option<&Tile> {
        let index = (pos.x, pos.y);
        self.tiles.at(index)
    }
    pub fn len(&self) -> u32 {
        self.tiles.len() as u32
    }
    pub fn dim(&self) -> (u32, u32) {
        self.tiles.dim()
    }
    pub fn each<F: FnMut(&Tile)>(&self, mut closure: F) {
        let elements = self.tiles.iter();
        for tile in elements {
            closure(tile);
        }
    }
    pub fn each_mut<F: FnMut(&Tile)>(&mut self, mut closure: F) {
        let mut elements = self.tiles.iter_mut();
        for tile in elements {
            closure(tile);
        }
    }
    pub fn each_with_pos<F: FnMut(Pnt2<u32>, &Tile)>(&self, mut closure: F) {
        let elements = self.tiles.indexed_iter();
        for pair in elements {
            let (coords, tile) = pair;
            let (x, y) = coords;
            let pos = Pnt2::<u32> {
                x: x,
                y: y,
            };
            closure(pos, tile);
        }
    }
}
