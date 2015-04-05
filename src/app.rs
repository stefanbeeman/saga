use graphics;
use opengl_graphics::{
    GlGraphics,
    OpenGL,
    Texture,
};
use piston;
use piston::event::*;
use piston::input::{Button, keyboard, mouse};
use sprite::*;
use std::cell::RefCell;

use render::Renderer;
use world::layer::Layer;

pub struct App {
    pub gl: GlGraphics,
    pub renderer: Renderer,
    pub scene: Scene<Texture>,
    pub world: Layer,
}

impl App {
    pub fn new(gl: GlGraphics) -> Self {
        let world = Layer::new();
        let renderer = Renderer::new();
        let scene = renderer.render_layer(&world);
        App{
            gl: gl,
            renderer: renderer,
            scene: scene,
            world: world,
        }
    }
    pub fn event(&mut self, e: Event) {
        use piston::event::*;
        self.scene.event(&e);
        if let Some(args) = e.render_args() {
            self.draw(args);
        }
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor(pos);
        }
        if let Some(button) = e.press_args() {
            self.press(button);
        }
    }
    pub fn draw(&mut self, args: RenderArgs) {
        use graphics::*;
        let mut scene = &mut self.scene;
        self.gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
            graphics::clear([1.0, 1.0, 1.0, 1.0], gl);
            scene.draw(c.transform, gl);
        });
    }
    pub fn cursor(&self, pos: [f64; 2]) {
        let (x, y) = (pos[0] as u32, pos[1] as u32);
        println!("{}, {}", x/32, y/32);
    }
    pub fn press(&self, button: Button) {
        match button {
            Button::Keyboard(key) => self.keyboard(key),
            Button::Mouse(button) => self.mouse(),
        }
    }
    pub fn keyboard(&self, key: keyboard::Key) {
        match key {
            keyboard::Key::Left => println!("Go Left!"),
            keyboard::Key::Right => println!("Go Right!"),
            keyboard::Key::Up => println!("Go Up!"),
            keyboard::Key::Down => println!("Go Down!"),
            _ => println!("Other key!"),
        }
    }
    pub fn mouse(&self) {
        println!("Mouse Button!")
    }
}
