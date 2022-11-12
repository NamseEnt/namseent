use geo::{Contains, Coordinate, EuclideanDistance};

#[derive(Debug)]
pub struct Circle {
    pub center: Coordinate<f64>,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: Coordinate<f64>, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn translate(&self, x: f64, y: f64) -> Self {
        Self {
            center: self.center + Coordinate { x, y },
            radius: self.radius,
        }
    }
}

impl Contains<Coordinate<f64>> for Circle {
    fn contains(&self, rhs: &Coordinate<f64>) -> bool {
        self.center.euclidean_distance(rhs) <= self.radius
    }
}
