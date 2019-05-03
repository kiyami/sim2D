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
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    let window_size = [800, 800];
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "gravitation",
            window_size
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();



    let center: [f64; 2] = [300.0, 300.0];
    let radius: f64 = 50.0;
    let n: u32 = 50;
    let pc: u32 = 1;
    let mut space = Space::new(&window_size, GlGraphics::new(opengl));
    space.fill(&center, radius, n, pc);

    let center: [f64; 2] = [500.0, 500.0];
    let radius: f64 = 50.0;
    let n: u32 = 50;
    let pc: u32 = 2;
    space.fill(&center, radius, n, pc);

    let center: [f64; 2] = [500.0, 300.0];
    let radius: f64 = 50.0;
    let n: u32 = 50;
    let pc: u32 = 3;
    space.fill(&center, radius, n, pc);
    //let mut space = Space::fill(&window_size, 10, GlGraphics::new(opengl));

    space.info();



    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            space.render(&r);
        }

        if let Some(u) = e.update_args() {
            space.update(&u);
        }
    }
}
