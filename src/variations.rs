use util::math::RealPoint;

#[derive(Clone, Debug)]
pub enum VariationType {
    Linear,
    Spiral,
}

#[derive(Clone, Debug)]
pub struct Variation {
    pub variation_type: VariationType,
    pub weight: f64,
}

impl Variation {
    pub fn apply(&self, point: RealPoint) -> RealPoint {
        match self.variation_type {
            VariationType::Linear => linear(point),
            VariationType::Spiral => spiral(point),
        }
    }
}


fn linear(point: RealPoint) -> RealPoint {
    point
}

fn spiral(RealPoint(x, y): RealPoint) -> RealPoint {
    let r = radius(x, y);
    let t = theta(x, y);
    RealPoint(
        1.0 / r * (t.cos() + r.sin()),
        1.0 / r * (t.sin() - r.cos()),
    )
}

fn radius(x: f64, y: f64) -> f64 {
    (x.powi(2) + y.powi(2)).sqrt()
}

fn theta(x: f64, y: f64) -> f64 {
    let t = x / y;
    if t.is_nan() {
        0.0
    } else {
        t.atan()
    }
}


#[derive(Clone, Debug)]
pub struct Variations {
    variations: Vec<Variation>,
    total_weight: f64,
}

impl Variations {
    pub fn new(variations: Vec<Variation>) -> Self {
        let total_weight: f64 = variations.iter()
            .map(|&Variation { weight, .. }| weight)
            .sum();

        Variations { variations, total_weight }
    }

    pub fn get_variation(&self, seed: f64) -> &Variation {
        assert!(seed >= 0.0, "seed should in [0, 1) range");
        let scaled_seed = seed * self.total_weight;
        let mut accumulated_weight = 0.0;
        for variation in self.variations.iter() {
            accumulated_weight += variation.weight;
            if accumulated_weight > scaled_seed {
                return variation;
            }
        }
        panic!("Seed is greater than 1")
    }
}