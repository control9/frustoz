use super::radius;
use super::theta;
use super::Variation;
use super::Variation::*;
use util::math::EPSILON;
use util::math::RealPoint;

impl Variation {
    pub fn apply(&self, point: &RealPoint) -> RealPoint {
        match self {
            Linear(w) => linear(point, *w),
            Spiral(w) => spiral(point, *w),
            Sinusoidal(w) => sinusoidal(point, *w),
        }
    }
}

fn linear(&RealPoint(x, y): &RealPoint, w: f64) -> RealPoint {
    RealPoint(w * x, w * y)
}

fn sinusoidal(&RealPoint(x, y): &RealPoint, w: f64) -> RealPoint {
    RealPoint(w * x.sin(), w * y.sin())
}

fn spiral(&RealPoint(x, y): &RealPoint, w: f64) -> RealPoint {
    let r = radius(x, y) + EPSILON;
    let t = theta(x, y);
    RealPoint(
        w / r * (t.cos() + r.sin()),
        w / r * (t.sin() - r.cos()),
    )
}
