extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use rand::Rng;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod particle;
mod vector;
mod space;

use space::Space;
use particle::Particle;
use vector::Vector;

fn main() {
    let mut space = Space::fill([600, 600], 10);
    let mut p1 = Particle::new(
        Vector::new(0.0, 0.0),
        Vector::new(0.0, 0.0),
        Vector::new(0.0, 0.0),
        1.0,
        [0.0, 0.0, 0.0, 1.0],);

    space.info();
}
