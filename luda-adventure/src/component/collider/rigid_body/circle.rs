use geo::{Contains, Coord, EuclideanDistance};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Circle {
    pub center: Coord<f64>,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: Coord<f64>, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn translate(&self, x: f64, y: f64) -> Self {
        Self {
            center: self.center + Coord { x, y },
            radius: self.radius,
        }
    }
}

impl Contains<Coord<f64>> for Circle {
    fn contains(&self, rhs: &Coord<f64>) -> bool {
        self.center.euclidean_distance(rhs) <= self.radius
    }
}
