
use crate::vector::Vector;
//use crate::force::Gravity;

pub struct Particle {
    pub position: Vector,
    pub velocity: Vector,
    pub acceleration: Vector,
    pub force: Vector,
    pub mass: f64,
    pub radius: f64,
    pub color: [f32; 4],
}

impl Particle {
    pub const velocity_limits: [f64; 2] = [-5.0, 5.0];
    pub const mass_limits: [f64; 2] = [1.0, 500.0];
    pub const radius_limits: [f64; 2] = [2.0, 5.0];

    pub fn new(pos: Vector, vel: Vector, acc: Vector, f: Vector, m: f64, r: f64, c: [f32; 4]) -> Particle {
        Particle {
            position: pos,
            velocity: vel,
            acceleration: acc,
            force: f,
            mass: m,
            radius: r,
            color: c,
        }
    }

    pub fn update(&mut self, size: [u32; 2], force: &Vector) {
        if (self.position.x > (size[0] as f64)) || (self.position.x < 0.0) {
            self.velocity.x *= -1.0;
            //self.velocity.y *= 0.8;
        }
        if (self.position.y > (size[1] as f64)) || (self.position.y < 0.0) {
            self.velocity.y *= -1.0;
            //self.velocity.x *= 0.8;
        }

        let dt = 1.5e-1;

        self.position.x += self.velocity.x * dt + (0.5 * self.acceleration.x * dt.powf(2.0));
        self.position.y += self.velocity.y * dt + (0.5 * self.acceleration.y * dt.powf(2.0));

        self.velocity.x += self.acceleration.x * dt;
        self.velocity.y += self.acceleration.y * dt;

        self.force.x = force.x;
        self.force.y = force.y;

        self.acceleration.x = self.force.x / self.mass;
        self.acceleration.y = self.force.y / self.mass;

    }
}
