// Utilities for rendering things. Trying to abstact away the actual graphics layer.
use na::{ Pnt2, Vec2 };
use opengl_graphics::Texture;
use uuid::Uuid;
use sprite::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::rc::Rc;

use world::layer::Layer;

pub struct Renderer {
    textures: HashMap<String, Rc<Texture>>,
    pub sprites: Vec<Uuid>,
    pub scene: Scene<Texture>,
}

impl Renderer {
    pub fn new(world: &Layer) -> Self {
        let mut cache = HashMap::new();
        match fs::walk_dir(&Path::new("gfx")) {
            Err(why) => panic!("Could not load textures"),
            Ok(paths) => for wrapped_path in paths {
                let path = wrapped_path.unwrap().path();
                let maybe_filename = path.to_str();
                match maybe_filename {
                    None => panic!("Attempted to load texture without filename (?)"),
                    Some(filename) => if filename.contains(".png") {
                        let tex = Rc::new(Texture::from_path(&path).unwrap());
                        cache.insert(filename.to_string(), tex);
                    }
                }
            }
        }
        let mut renderer = Renderer {
            textures: cache,
            sprites: Vec::<Uuid>::new(),
            scene: Scene::new(),
        };
        renderer.render_layer(world);
        renderer
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
        let sprite = Sprite::from_texture(tex);
        sprite
    }
    pub fn move_anchor(&mut self, delta: Vec2<i32>) {
        let mut sprites = &self.sprites;
        for id in sprites {
            let mut sprite = self.scene.child_mut(&id).unwrap();
            let (x, y) = sprite.position();
            sprite.set_position(x + (delta.x as f64), y + (delta.y as f64));
        }
    }
}

pub mod tile;
pub mod layer;
