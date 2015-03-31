// Utilities for rendering things. Trying to abstact away the actual graphics layer.

use na::{ Pnt2, Vec2 };
use opengl_graphics::Texture;
use uuid::Uuid;
use sprite::*;
use std::path::Path;
use std::rc::Rc;

pub fn create_sprite(path: &str) -> Sprite<Texture> {
    let tilePath = "./gfx/".to_string() + path + ".png";
    let tex = Path::new(&tilePath);
    let tex = Rc::new(Texture::from_path(&tex).unwrap());
    Sprite::from_texture(tex.clone())
}

pub mod tile;
pub mod layer;
