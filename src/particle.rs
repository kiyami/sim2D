
use crate::vector::Vector;
//use crate::force::Gravity;

pub struct Particle {
    pub position: Vector,
    pub velocity: Vector,
    pub acceleration: Vector,
    pub force: Vector,
    pub mass: f64,
    pub color: [f64; 4],
}

impl Particle {
    pub fn new(pos: Vector, vel: Vector, acc: Vector, f: Vector, m: f64, c: [f64; 4]) -> Particle {
        Particle {
            position: pos,
            velocity: vel,
            acceleration: acc,
            force: f,
            mass: m,
            color: c,
        }
    }

    pub const velocity_limits: [f64; 2] = [-20.0, 20.0];
    pub const mass_limits: [f64; 2] = [1.0, 200.0];


    pub fn update(&mut self, size: [u32; 2]) {
        if (self.position.x > (size[0] as f64)) || (self.position.x < 0.0) {
            self.velocity.x *= -1.0;
        }
        if (self.position.y > (size[1] as f64)) || (self.position.y < 0.0) {
            self.velocity.y *= -1.0;
        }

        let dt = 1.0e-1;

        self.position.x += self.velocity.x * dt + (0.5 * self.acceleration.x * dt.powf(2.0));
        self.position.y += self.velocity.y * dt + (0.5 * self.acceleration.y * dt.powf(2.0));

        self.velocity.x += self.acceleration.x * dt;
        self.velocity.y += self.acceleration.y * dt;

    }
}
