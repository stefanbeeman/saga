#![feature(fs_walk)]

extern crate ai_behavior;
extern crate graphics;
extern crate rand;
extern crate nalgebra as na;
extern crate ndarray as nd;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;
extern crate sprite;
extern crate uuid;

use ai_behavior::{
    Action,
    Sequence,
    Wait,
    WaitForever,
    While,
};
use opengl_graphics::{
    GlGraphics,
    OpenGL,
    Texture,
};
use sdl2_window::Sdl2Window;
use sprite::*;
use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;

mod world;
mod render;

fn start_game(width: u32, height: u32, fullscreen: bool) {
    let opengl = OpenGL::_3_2;
    let window = Sdl2Window::new(
        opengl,
        piston::window::WindowSettings {
            title: "saga".to_string(),
            size: [width, height],
            fullscreen: fullscreen,
            exit_on_esc: true,
            samples: 0,
        }
    );
    let window = RefCell::new(window);
    let mut gl = GlGraphics::new(opengl);
    let renderer = render::Renderer::new();

    let game_world = world::layer::Layer::new();
    let mut scene = renderer.render_layer(game_world);

    for e in piston::events(&window) {
        use piston::event::*;
        scene.event(&e);
        if let Some(args) = e.render_args() {
            use graphics::*;
            gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
                graphics::clear([1.0, 1.0, 1.0, 1.0], gl);
                scene.draw(c.transform, gl);
            });
        }
        if let Some(_) = e.press_args() {
            println!("Button pressed!");
        }
    }
}

fn main() {
    start_game(1440, 1280, false);
}
