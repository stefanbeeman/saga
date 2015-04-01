//Utilities for rendering layers

use na::{ Pnt2, Vec2 };
use opengl_graphics::Texture;
use uuid::Uuid;
use sprite::*;
use std::path::Path;
use std::rc::Rc;

use render::Renderer;
use render::tile;
use world::layer::Layer;
use world::tile::Tile;

impl Renderer {
    pub fn render_layer(&self, layer: Layer) -> Scene<Texture> {
        let mut scene = Scene::new();
        layer.each_with_pos(|pos: Pnt2<u32>, tile: &Tile| {
            let mut sprite = self.render_tile(tile);
            sprite.set_anchor(0 as f64, 0 as f64);
            sprite.set_position((pos.x * 32) as f64, (pos.y * 32) as f64);
            scene.add_child(sprite);
        });
        scene
    }
}
