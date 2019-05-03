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
}

impl Space {
    pub fn new(size: &[u32; 2], gl: GlGraphics) -> Space {
        let mut empty_particle_vector: Vec<Particle> = vec![];
        Space {
            size: *size,
            particles: empty_particle_vector,
            n: 0u32,
            gl: gl
        }
    }

    pub fn fill(&mut self, center: &[f64; 2], radius: f64, n: u32, pc: u32) {
        let mut rng = rand::thread_rng();
        let pi = std::f64::consts::PI;

        let init_pos = Vector::new(center[0], center[1]);
        let init_vel = Vector::new(0.0, 0.0);
        let init_acc = Vector::new(0.0, 0.0);
        let init_f = Vector::new(0.0, 0.0);
        let init_m = 5_000.0;
        let init_r = 8.0;
        let init_c = [0.5, 0.1, 0.1, 1.0];
        let mut initial_particle = Particle::new(init_pos, init_vel, init_acc, init_f, init_m, init_r, init_c);
        self.particles.push(initial_particle);
        self.n += 1;

        for i in 0..n {
            let rand_angle = rng.gen_range(0.0, 2.0 * pi);
            let rand_radius = rng.gen_range(0.0, radius);

            let x = center[0] + (rand_radius * rand_angle.sin());
            let y = center[1] + (rand_radius * rand_angle.cos());

            let vx = rng.gen_range(Particle::velocity_limits[0], Particle::velocity_limits[1]);
            let vy = rng.gen_range(Particle::velocity_limits[0], Particle::velocity_limits[1]);

            let m = rng.gen_range(Particle::mass_limits[0], Particle::mass_limits[1]);

            let pos = Vector::new(x, y);
            let vel = Vector::new(vx, vy);
            let acc = Vector::zero();
            let f = Vector::zero();

            // particle radius related to mass
            let del_m = Particle::mass_limits[1] - Particle::mass_limits[0];
            let del_r = Particle::radius_limits[1] - Particle::radius_limits[0];

            let r = ((m - Particle::mass_limits[0]) * (del_r / del_m)) + Particle::radius_limits[0];

            // particle color related to mass
            let mut c = [0.0, 0.0, 0.0, 1.0];
            let color_weight = (m - Particle::mass_limits[0])/(Particle::mass_limits[1] - Particle::mass_limits[0]);
            if pc == 1 {
                c = [(color_weight as f32)*0.1, (color_weight as f32)*0.6, (color_weight as f32)*0.6, 1.0];
            } else if pc == 2 {
                c = [(color_weight as f32)*0.6, (color_weight as f32)*0.2, (color_weight as f32)*0.1, 1.0];
            } else if pc == 3 {
                c = [(color_weight as f32)*0.1, (color_weight as f32)*0.6, (color_weight as f32)*0.1, 1.0];
            }

            self.particles.push(Particle::new(pos, vel, acc, f, m, r, c));
            self.n += 1;
        }
    }

    pub fn fill_2(size: &[u32; 2], n: u32, gl: GlGraphics) -> Space {
        let mut rng = rand::thread_rng();
        let mut temp_particles: Vec<Particle> = vec![];

        let init_pos = Vector::new((size[0] as f64)/2.0, (size[1] as f64)/2.0);
        let init_vel = Vector::new(0.0, 0.0);
        let init_acc = Vector::new(0.0, 0.0);
        let init_f = Vector::new(0.0, 0.0);
        let init_m = 1_500.0;
        let init_r = 7.0;
        let init_c = [0.5, 0.1, 0.1, 1.0];
        let mut initial_particle = Particle::new(init_pos, init_vel, init_acc, init_f, init_m, init_r, init_c);
        temp_particles.push(initial_particle);

        for i in 0..(n-1) {
            let rand_x = rng.gen_range((size[0] as f64) * 0.4, (size[0] as f64) * 0.6);
            let rand_y = rng.gen_range((size[1] as f64) * 0.4, (size[1] as f64) * 0.6);

            let rand_vx = rng.gen_range(Particle::velocity_limits[0], Particle::velocity_limits[1]);
            let rand_vy = rng.gen_range(Particle::velocity_limits[0], Particle::velocity_limits[1]);

            let rand_m = rng.gen_range(Particle::mass_limits[0], Particle::mass_limits[1]);

            let pos = Vector::new(rand_x, rand_y);
            let vel = Vector::new(rand_vx, rand_vy);
            //let vel = Vector::new(0.0, 0.0);
            let acc = Vector::new(0.0, 0.0);
            let f = Vector::new(0.0, 0.0);
            let m = rand_m;

            let del_m = Particle::mass_limits[1] - Particle::mass_limits[0];
            let del_r = Particle::radius_limits[1] - Particle::radius_limits[0];

            let r = ((m - Particle::mass_limits[0]) * (del_r / del_m)) + Particle::radius_limits[0];

            let color_weight = (m - Particle::mass_limits[0])/(Particle::mass_limits[1] - Particle::mass_limits[0]);
            let c = [(color_weight as f32)*0.1, (color_weight as f32)*0.6, (color_weight as f32)*0.6, 1.0];
            temp_particles.push(Particle::new(pos, vel, acc, f, m, r, c));
        }
        Space { size: *size, particles: temp_particles, n: n, gl: gl}
    }

    pub fn info(&self) {
        println!("Size of the space: [{},{}] (pixel)", self.size[0], self.size[1]);
        println!("Number of particles: {}", self.particles.len());

        for p in self.particles.iter() {
            println!("Position: [{},{}]", p.position.x, p.position.y);
        }
    }

    pub fn calculate_force(&self) -> Vec<Vector> {
        let G = 6.6740831e-11; // m3 kg-1 s-2
        let e = 0.2;
        let mut force_list: Vec<Vector> = vec![];
        for p1 in &self.particles {
            let mut single_force: Vector = Vector::new(0.0, 0.0);
            for p2 in &self.particles {
                let con = G * p1.mass * p2.mass;
                let temp_dist: Vector = Vector::distance(&p1.position, &p2.position);
                if temp_dist.length() !=  0.0 {
                    let temp_force_x = con * temp_dist.sin() / temp_dist.length();
                    let temp_force_y = con * temp_dist.cos() / temp_dist.length();
                    single_force.add_to_self(&Vector::new(temp_force_x, temp_force_y));
                } else {
                    single_force.add_to_self(&Vector::new(0.0, 0.0));
                }
            }
            //let mut single_force: Vector = Vector::new(20.0, 20.0);
            force_list.push(single_force);
        }
        force_list
    }


    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const ORANGE:   [f32; 4] = [0.9, 0.8, 0.6, 0.8];
        const BLUE:   [f32; 4] = [0.5, 0.7, 0.8, 0.8];
        const BLACK:   [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE:   [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        //clear(BLUE, &mut self.gl);
        clear([0.80, 0.85, 0.90, 1.0], &mut self.gl);

        for particle in self.particles.iter() {
            let radius = particle.radius;
            let square = rectangle::square(0.0, 0.0, radius);
            let (x, y) = (particle.position.x, particle.position.y);

            self.gl.draw(args.viewport(), |c, gl| {
                let transform = c.transform.trans(x, y);
                rectangle(particle.color, square, transform, gl);
            });
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        //let force = Vector::new(0.0, 0.0);
        let force = self.calculate_force();
        for i in 0..self.n {
            self.particles[i as usize].update(self.size, &force[i as usize]);
        }
    }
}
