extern crate graphics;

extern crate piston;
use piston::input::*;

extern crate opengl_graphics;
use opengl_graphics::{ GlGraphics, OpenGL };

extern crate rand;
use rand::Rng;

use crate::particle::Particle;
use crate::vector::Vector;

pub struct Space {
    pub size: [u32; 2],
    pub particles: Vec<Particle>,
    pub n: u32,
    pub gl: GlGraphics,

    pub distance_table: Vec<Vec<f64>>,
}

impl Space {
    pub fn fill(size: [u32; 2], n: u32, gl: GlGraphics) -> Space {
        let mut rng = rand::thread_rng();
        let mut temp_particles: Vec<Particle> = vec![];
        for i in 0..n {
            let rand_x = rng.gen_range(0.0, size[0] as f64);
            let rand_y = rng.gen_range(0.0, size[1] as f64);

            let rand_vx = rng.gen_range(Particle::velocity_limits[0], Particle::velocity_limits[1]);
            let rand_vy = rng.gen_range(Particle::velocity_limits[0], Particle::velocity_limits[1]);

            let rand_m = rng.gen_range(Particle::mass_limits[0], Particle::mass_limits[1]);

            let pos = Vector::new(rand_x, rand_y);
            let vel = Vector::new(rand_vx, rand_vy);
            let acc = Vector::new(0.0, 0.0);
            let f = Vector::new(0.0, 0.0);
            let m = rand_m;
            let c = [0.0, 0.0, 0.0, 1.0];
            temp_particles.push(Particle::new(pos, vel, acc, f, m, c));
        }
        Space { size: size, particles: temp_particles, n: n, gl: gl, distance_table: vec![vec![]]}
    }

    pub fn info(&self) {
        println!("Size of the space: [{},{}] (pixel)", self.size[0], self.size[1]);
        println!("Number of particles: {}", self.particles.len());

        for p in self.particles.iter() {
            println!("Position: [{},{}]", p.position.x, p.position.y);
        }
    }

    pub fn action(&self) {
        let G = 1.0;
        let e = 1.0;
        let mut force_list: Vec<Vector> = vec![];
        for p1 in &self.particles {
            let mut single_force: Vector = Vector::new(0.0, 0.0);
            for p2 in &self.particles {
                let con = G * p1.mass * p2.mass;
                let temp_dist = Vector::distance(&p1.position, &p2.position);

                let temp_force_x = con * temp_dist.sin() / temp_dist.length();
                let temp_force_y = con * temp_dist.cos() / temp_dist.length();

                single_force.add_to_self(&Vector::new(temp_force_x, temp_force_y));
            }
            single_force.info();
            force_list.push(single_force);
        }

    }


    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const ORANGE:   [f32; 4] = [0.9, 0.8, 0.6, 0.8];
        const BLUE:   [f32; 4] = [0.5, 0.7, 0.8, 0.7];
        const BLACK:   [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        clear(BLUE, &mut self.gl);

        for particle in self.particles.iter() {
            let square = rectangle::square(0.0, 0.0, 10.0);
            let (x, y) = (particle.position.x, particle.position.y);

            self.gl.draw(args.viewport(), |c, gl| {
                let transform = c.transform.trans(x, y);
                rectangle(BLACK, square, transform, gl);
            });
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {

    }
}
