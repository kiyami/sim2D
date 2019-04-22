extern crate rand;
use rand::Rng;

use crate::particle::Particle;
use crate::vector::Vector;

pub struct Space {
    pub size: [u32; 2],
    pub particles: Vec<Particle>,
}

impl Space {
    pub fn fill(size: [u32; 2], n: u32) -> Space {
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
            let m = rand_m;
            let c = [0.0, 0.0, 0.0, 1.0];
            temp_particles.push(Particle::new(pos, vel, acc, m, c));
        }
        Space { size: size, particles: temp_particles }
    }

    pub fn info(&self) {
        println!("Size of the space: [{},{}] (pixel)", self.size[0], self.size[1]);
        println!("Number of particles: {}", self.particles.len());

        for p in self.particles.iter() {
            println!("Position: [{},{}]", p.position.x, p.position.y);
        }
    }
}
