extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::GlGraphics;

#[cfg(test)]
mod structs;
mod tests;

pub struct App {
    gl: GlGraphics,

}

fn main() {
    println!("Hello, (physics) world!");
}
