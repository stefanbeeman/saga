// Utilities for rendering things. Trying to abstact away the actual graphics layer.
use na::{ Pnt2, Vec2 };
use opengl_graphics::Texture;
use uuid::Uuid;
use sprite::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::rc::Rc;

pub struct Renderer {
    textures: HashMap<String, Rc<Texture>>,
}

impl Renderer {
    pub fn new() -> Self {
        let mut cache = HashMap::new();
        match fs::walk_dir(&Path::new("gfx")) {
            Err(why) => panic!("Could not load textures"),
            Ok(paths) => for wrapped_path in paths {
                let path = wrapped_path.unwrap().path();
                let maybe_filename = path.to_str();
                match maybe_filename {
                    None => panic!("Attempted to load texture without filename(?)"),
                    Some(filename) => if filename.contains(".png") {
                        let tex = Rc::new(Texture::from_path(&path).unwrap());
                        cache.insert(filename.to_string(), tex);
                    }
                }
            }
        }
        Renderer {
            textures: cache,
        }
    }
    pub fn get_texture(&self, path: &str) -> Rc<Texture> {
        match self.textures.get(path) {
            None => panic!("Attempted to use texture absent from cache"),
            Some(tex) => tex.clone(),
        }
    }
    pub fn get_sprite(&self, path: &str) -> Sprite<Texture> {
        let tile_path = "gfx/".to_string() + path + ".png";
        let tex = self.get_texture(&tile_path);
        Sprite::from_texture(tex)
    }
}

pub mod tile;
pub mod layer;
