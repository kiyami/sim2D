extern crate opengl_graphics;
extern crate rand;

pub mod particle {
    use std::collections::HashMap;
    use rand::Rng;
    use opengl_graphics::{ GlGraphics, OpenGL };

    pub struct Particle {
        pub potential_energy: f64,
        pub kinetic_energy: f64,

        pub x: f64,
        pub y: f64,
        pub vx: f64,
        pub vy: f64,

        pub ax: f64,
        pub ay: f64,
        pub fx: f64,
        pub fy: f64,

        pub mass: f64,
        pub r: f64,
        pub color: [f64; 4],
    }

    impl Particle {
        pub fn print_energy(&self) {
            println!("Kinetic Energy: {}", self.kinetic_energy);
            println!("Potential Energy: {}", self.potential_energy);
        }

        pub fn new() -> Particle {
            let mut rng = rand::thread_rng();
            let mut new_particle = Particle {
                potential_energy: 0.0,
                kinetic_energy: 0.0,
                x: rng.gen_range(200.0, 400.0),
                y: rng.gen_range(200.0, 400.0),
                vx: 0.0,
                vy: 0.0,
                ax: 0.0,
                ay: 0.0,
                fx: 0.0,
                fy: 0.0,
                mass: 100.0,
                r: 10.0,
                color: [0.0, 0.0, 0.0, 1.0],
            };
            new_particle
        }
    }

    pub struct AllParticles {
        particles: Vec<Particle>,
    }

    impl AllParticles {

        pub fn new(n: u32) -> AllParticles {
            let opengl = OpenGL::V3_2;

            let mut temp_particles: Vec<Particle>;
            for i in 0..n {
                temp_particles.push(Particle::new());
            }
            let mut particles = AllParticles {
                particles: temp_particles,
            };
            particles
        }


    }

}
