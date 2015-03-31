// Utility for rendering a single tile of floor
use opengl_graphics::Texture;
use sprite::*;
use render::create_sprite;
use world::tile::{ Floor, Tile };

fn floor_gfx_path(tile: &Tile) -> String {
    match tile.floor {
        Floor::Grass(variant) => {
            format!("tiles/floor/grass{}", variant)
        }
    }
}

pub fn render_floor(tile: &Tile) -> Sprite<Texture> {
    let path = floor_gfx_path(tile);
    create_sprite(&path)
}

pub fn render_tile(tile: &Tile) -> Sprite<Texture> {
    render_floor(tile)
}
