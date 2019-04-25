
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

    pub const velocity_limits: [f64; 2] = [0.0, 100.0];
    pub const mass_limits: [f64; 2] = [0.0, 10.0];

    pub fn move_it(&mut self, f: Vector) {
        let dt = 1.0;
        self.position.x += self.velocity.x * dt + 0.5 * self.acceleration.x * dt.powf(2.0);
        self.position.y += self.velocity.y * dt + 0.5 * self.acceleration.y * dt.powf(2.0);

        self.velocity.x += self.acceleration.x * dt;
        self.velocity.y += self.acceleration.y * dt;

        self.acceleration.x = f.x / self.mass;
        self.acceleration.y = f.y / self.mass;
    }
}
