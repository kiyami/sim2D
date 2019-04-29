
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Vector {
        Vector { x: x, y: y}
    }

    pub fn info(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }

    pub fn length(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powf(2.0) + self.y.powf(2.0)
    }

    pub fn sin(&self) -> f64 {
        self.x / self.length()
    }

    pub fn cos(&self) -> f64 {
        self.y / self.length()
    }

    pub fn add_to_self(&mut self, other: &Vector) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn sum(v1: &Vector, v2: &Vector) -> Vector {
        Vector {
            x: v1.x + v2.x,
            y: v1.y + v2.y,
        }
    }

    pub fn distance(v1: &Vector, v2: &Vector) -> Vector {
        let del_x = v2.x - v1.x;
        let del_y = v2.y - v1.y;

        Vector {
            x: del_x,
            y: del_y,
        }
    }
}
