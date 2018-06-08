use util::math::RealPoint;
use util::math::EPSILON;

mod types;

#[derive(Clone, Debug)]
pub enum Variation {
    Linear(f64),
    Spiral(f64),
}

fn radius(x: f64, y: f64) -> f64 {
    (x.powi(2) + y.powi(2)).sqrt()
}

fn theta(x: f64, y: f64) -> f64 {
    x.atan2(y)
}


#[derive(Clone, Debug)]
pub struct Variations {
    variations: Vec<Variation>,
}

impl Variations {
    pub fn new(variations: Vec<Variation>) -> Self {
        Variations { variations}
    }

    pub fn apply(&self, point: &RealPoint) -> RealPoint {
        self.variations.iter()
            .map(|var| var.apply(point))
            .sum()
    }
}