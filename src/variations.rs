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
    pub fn apply(&self, point: &RealPoint) -> RealPoint {
        match self.variation_type {
            VariationType::Linear => linear(point),
            VariationType::Spiral => spiral(point),
        }
    }
}


fn linear(&RealPoint(x, y): &RealPoint) -> RealPoint {
    RealPoint(x, y)
}

fn spiral(&RealPoint(x, y): &RealPoint) -> RealPoint {
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
}

impl Variations {
    pub fn new(variations: Vec<Variation>) -> Self {
        Variations { variations}
    }

    pub fn apply(&self, point: &RealPoint) -> RealPoint {
        self.variations.iter()
            .map(|var| var.weight * var.apply(point))
            .sum()
    }
}