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

mod lib;
use crate::lib::particle::Particle;
use crate::lib::particle::AllParticles;

fn main() {

    const GREEN: [f64; 4] = [0.0, 1.0, 0.5, 1.0];
    const RED:   [f64; 4] = [1.0, 0.4, 0.0, 1.0];
    const BLUE:   [f64; 4] = [0.2, 0.4, 0.7, 1.0];
    const ORANGE:   [f64; 4] = [0.9, 0.8, 0.6, 0.8];
    const BLACK_SOFT:   [f64; 4] = [0.0, 0.0, 0.0, 0.5];
    const BLACK:   [f64; 4] = [0.0, 0.0, 0.0, 1.0];

    let mut rng = rand::thread_rng();
    let p1 = Particle {
        potential_energy: 0.0,
        kinetic_energy: 0.0,
        x: rng.gen_range(295.0, 305.0),
        y: rng.gen_range(295.0, 305.0),
        vx: 0.0,
        vy: 0.0,
        ax: 0.0,
        ay: 0.0,
        fx: 0.0,
        fy: 0.0,
        mass: 100.0,
        r: 10.0,
        color: BLACK,
    };

    p1.print_energy();

    let mut particles = AllParticles.new(10);

    }

}
