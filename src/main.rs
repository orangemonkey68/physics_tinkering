extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::GlGraphics;

mod structs;

pub struct App {
    gl: GlGraphics,

}

fn main() {
    println!("Hello, (physics) world!");
}
