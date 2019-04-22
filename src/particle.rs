use crate::vector::Vector;

pub struct Particle {
    pub position: Vector,
    velocity: Vector,
    acceleration: Vector,
    mass: f64,
    color: [f64; 4],
}

impl Particle {
    pub fn new(pos: Vector, vel: Vector, acc: Vector, m: f64, c: [f64; 4]) -> Particle {
        Particle {
            position: pos,
            velocity: vel,
            acceleration: acc,
            mass: m,
            color: c,
        }
    }

    pub const velocity_limits: [f64; 2] = [0.0, 100.0];
    pub const mass_limits: [f64; 2] = [0.0, 10.0];

}
