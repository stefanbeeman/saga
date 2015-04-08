use graphics;
use na::{ Pnt2, Vec2 };
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
    pub cursor: Sprite<Texture>,
    pub gl: GlGraphics,
    pub renderer: Renderer,
    pub world: Layer,
}

impl App {
    pub fn new(gl: GlGraphics) -> Self {
        let world = Layer::new();
        let renderer = Renderer::new(&world);
        let mut cursor = renderer.get_sprite("ui/cursor/cursor");
        cursor.set_anchor(0 as f64, 0 as f64);
        cursor.set_position(0 as f64, 0 as f64);
        App{
            cursor: cursor,
            gl: gl,
            renderer: renderer,
            world: world,
        }
    }
    pub fn event(&mut self, e: Event) {
        use piston::event::*;
        self.renderer.scene.event(&e);
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
        let mut scene = &mut self.renderer.scene;
        let mut cursor = &mut self.cursor;
        self.gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
            graphics::clear([1.0, 1.0, 1.0, 1.0], gl);
            scene.draw(c.transform, gl);
            cursor.draw(c.transform, gl);
        });
    }
    pub fn cursor(&mut self, pos: [f64; 2]) {
        let x = (pos[0]/32.0).floor() * 32.0;
        let y = (pos[1]/32.0).floor() * 32.0;
        self.cursor.set_position(x as f64, y as f64);
    }
    pub fn press(&mut self, button: Button) {
        match button {
            Button::Keyboard(key) => self.keyboard(key),
            Button::Mouse(button) => self.mouse(),
        }
    }
    pub fn keyboard(&mut self, key: keyboard::Key) {
        match key {
            keyboard::Key::Left => self.renderer.move_anchor(Vec2::<i32> {
                x: 32,
                y: 0,
            }),
            keyboard::Key::Right => self.renderer.move_anchor(Vec2::<i32> {
                x: -32,
                y: 0,
            }),
            keyboard::Key::Up => self.renderer.move_anchor(Vec2::<i32> {
                x: 0,
                y: 32,
            }),
            keyboard::Key::Down => self.renderer.move_anchor(Vec2::<i32> {
                x: 0,
                y: -32,
            }),
            _ => println!("Other key!"),
        }
    }
    pub fn mouse(&self) {
        println!("Mouse Button!")
    }
}
