use std::f64::consts::PI;
use super::rad2;
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
            Sinusoidal(w) => sinusoidal(point, *w),
            Spherical(w) => spherical(point, *w),
            Swirl(w) => swirl(point, *w),
            Horseshoe(w) => horseshoe(point, *w),
            Polar(w) => polar(point, *w),
            Handkerchief(w) => handkerchief(point, *w),
            Heart(w) => heart(point, *w),
            Disc(w) => disc(point, *w),
            Spiral(w) => spiral(point, *w),
        }
    }
}

fn linear(&RealPoint(x, y): &RealPoint, w: f64) -> RealPoint {
    RealPoint(w * x, w * y)
}

fn sinusoidal(&RealPoint(x, y): &RealPoint, w: f64) -> RealPoint {
    RealPoint(w * x.sin(), w * y.sin())
}

fn spherical(&RealPoint(x, y): &RealPoint, w: f64) -> RealPoint {
    let r2 = 1.0 / (rad2(x, y) + EPSILON);
    RealPoint(w * r2 * x, w * r2 * y)
}

fn swirl(&RealPoint(x, y): &RealPoint, w: f64) -> RealPoint {
    let r2 = rad2(x, y);
    let c1 = r2.sin();
    let c2 = r2.cos();
    RealPoint(
        w * (c1 * x - c2 * y),
        w * (c2 * x - c1 * y),
    )
}

fn horseshoe(&RealPoint(x, y): &RealPoint, w: f64) -> RealPoint {
    let inv_r = 1.0 / (radius(x, y) + EPSILON);
    RealPoint(
        w * (x - y) * (x + y) * inv_r,
        w * 2.0 * x * y * inv_r,
    )
}

fn polar(&RealPoint(x, y): &RealPoint, w: f64) -> RealPoint {
    let r = radius(x, y);
    let a = theta(x, y);
    RealPoint(w * a / PI, w * (r - 1.0))
}

fn handkerchief(&RealPoint(x, y): &RealPoint, w: f64) -> RealPoint {
    let r = radius(x, y);
    let t = theta(x, y);

    RealPoint(w * r * (t + r).sin(), w * r * (t - r).cos())
}

fn heart(&RealPoint(x, y): &RealPoint, w: f64) -> RealPoint {
    let r = radius(x, y);
    let a = r * theta(x, y);

    RealPoint(w * r * a.sin(), -w * r * a.cos())
}

fn disc(&RealPoint(x, y): &RealPoint, w: f64) -> RealPoint {
    let a = theta(x * PI, y * PI) / PI;
    let r = PI * radius(x, y);
    RealPoint(w * r.sin() * a, w * r.cos() * a)
}

fn spiral(&RealPoint(x, y): &RealPoint, w: f64) -> RealPoint {
    let r = radius(x, y) + EPSILON;
    let t = theta(x, y);
    RealPoint(
        w / r * (t.cos() + r.sin()),
        w / r * (t.sin() - r.cos()),
    )
}

fn hyperbolic(&RealPoint(x, y): &RealPoint, w: f64) -> RealPoint {
    let r = radius(x, y) + EPSILON;
    let t = theta(x, y);
    RealPoint(w * a.sin() / r, w * a.cos() * r)
}

fn diamond(&RealPoint(x, y): &RealPoint, w: f64) -> RealPoint {
    let r = radius(x, y);
    let t = theta(x, y);
    RealPoint(w * a.sin() * r.cos(), w * a.cos() * r.sin())
}
