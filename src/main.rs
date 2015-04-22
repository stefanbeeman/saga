#![feature(convert, fs_walk)]
extern crate ai_behavior;
extern crate graphics;
extern crate rand;
extern crate nalgebra as na;
extern crate ndarray as nd;
extern crate opengl_graphics;
extern crate piston;
extern crate probability;
extern crate sdl2_window;
extern crate sprite;
extern crate uuid;

mod app;
mod game;
mod render;
mod world;

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
use piston::event::*;
use piston::window::{ WindowSettings, Size };

fn game_loop() {
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
    for e in window.events() {
        use piston::event::*;
        game.event(e);
    }
}

fn main() {
    let mut xa = 0.0;
    for _ in 0..1000 {
        xa += game::test::test(6.0, 6.0);
    }
    xa = xa/1000.0;
    println!("Equal {}", xa);

    let mut xb = 0.0;
    for _ in 0..1000 {
        xb += game::test::test(3.0, 6.0);
    }
    xb = xb/1000.0;
    println!("Lower {}", xb);

    let mut xc = 0.0;
    for _ in 0..1000 {
        xc += game::test::test(6.0, 3.0);
    }
    xc = xc/1000.0;
    println!("Higher {}", xc);
    //
    // let mut equal = 0.0;
    // for n in 0..1000 {
    //     equal += game::test::test(6.0, 6.0);
    // }
    // equal = equal/1000;
    //
    // let mut higher = 0.0;
    // for n in 0..1000 {
    //     higher += game::test::test(6.0, 3.0);
    // }
    // higher = higher/1000;
    //
    // println!("Lower {}", lower);
    // println!("Equal {}", equal);
    // println!("Higher {}", higher);
    //game_loop();
}
