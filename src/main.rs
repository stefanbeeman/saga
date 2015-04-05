#![feature(convert, fs_walk)]
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

mod app;
mod world;
mod render;

use std::cell::RefCell;
use std::rc::Rc;

use sprite::*;
use ai_behavior::{
    Action,
    Sequence,
    Wait,
    WaitForever,
    While,
};

use sdl2_window::Sdl2Window;
use opengl_graphics::{
    GlGraphics,
    OpenGL,
    Texture,
};
use piston::window::{ WindowSettings, Size };

fn main() {
    let opengl = OpenGL::_3_2;
    let (width, height) = (1440, 1280);
    let window = Sdl2Window::new(
        opengl,
        WindowSettings::new(
            "saga".to_string(),
            Size { width: width, height: height }
        )
        .exit_on_esc(true)
    );
    let window = Rc::new(RefCell::new(window));
    let mut gl = GlGraphics::new(opengl);
    let mut game = app::App::new(gl);
    for e in piston::events(window) {
        use piston::event::*;
        game.event(e);
    }
}
