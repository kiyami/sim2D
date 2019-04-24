use crate::particle::Particle;
use crate::vector::Vector;

pub struct Gravity {
    pub G: f64,
    pub e: f64,
}

impl Gravity {
    pub fn new(G: f64, e: f64) -> Gravity {
        Gravity {
            G: G,
            e: e,
        }
    }

}
