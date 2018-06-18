use util::math::RealPoint;
use rand::prelude::*;

mod types;

#[derive(Clone, Debug)]
pub enum Variation {
    Linear(f64),
    Sinusoidal(f64),
    Spherical(f64),
    Swirl(f64),
    Horseshoe(f64),
    Polar(f64),
    Handkerchief(f64),
    Heart(f64),
    Disc(f64),
    Spiral(f64),
    Hyperbolic(f64),
    Diamond(f64),
    Julia(f64)
}

fn radius(x: f64, y: f64) -> f64 {
    (x.powi(2) + y.powi(2)).sqrt()
}

fn rad2(x: f64, y: f64) -> f64 {
    x.powi(2) + y.powi(2)
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

    pub fn apply<R: Rng + ?Sized>(&self, point: &RealPoint, rng: &mut R) -> RealPoint {
        self.variations.iter()
            .map(|var| var.apply(point, rng))
            .sum()
    }
}