// Utility for rendering a single tile of floor
use opengl_graphics::Texture;
use sprite::*;
use render::Renderer;
use world::tile::{ Floor, Tile };

fn floor_gfx_path(tile: &Tile) -> String {
    match tile.floor {
        Floor::Grass(variant) => {
            format!("tiles/floor/grass{}", variant)
        }
    }
}

impl Renderer {
    pub fn render_floor(&self, tile: &Tile) -> Sprite<Texture> {
        let path = floor_gfx_path(tile);
        self.get_sprite(&path)
    }
    pub fn render_tile(&self, tile: &Tile) -> Sprite<Texture> {
        self.render_floor(tile)
    }
}
