use super::Variation;
use super::Variation::*;
use crate::util::math::{rad2, radius, theta, RealPoint, EPSILON};
use rand::Rng;
use std::f64::consts::PI;

impl Variation {
    pub fn apply<R: Rng + Sized>(&self, point: &RealPoint, rng: &mut R) -> RealPoint {
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
            Hyperbolic(w) => hyperbolic(point, *w),
            Diamond(w) => diamond(point, *w),
            Julia(w) => julia(point, *w, rng.gen()),
            JuliaN(w, power, dist) => julia_n(point, *w, *power, *dist, rng),
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
    RealPoint(w * (c1 * x - c2 * y), w * (c2 * x - c1 * y))
}

fn horseshoe(&RealPoint(x, y): &RealPoint, w: f64) -> RealPoint {
    let inv_r = 1.0 / (radius(x, y) + EPSILON);
    RealPoint(w * (x - y) * (x + y) * inv_r, w * 2.0 * x * y * inv_r)
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
    RealPoint(w / r * (t.cos() + r.sin()), w / r * (t.sin() - r.cos()))
}

fn hyperbolic(&RealPoint(x, y): &RealPoint, w: f64) -> RealPoint {
    let r = radius(x, y) + EPSILON;
    let a = theta(x, y);
    RealPoint(w * a.sin() / r, w * a.cos() * r)
}

fn diamond(&RealPoint(x, y): &RealPoint, w: f64) -> RealPoint {
    let r = radius(x, y);
    let a = theta(x, y);
    RealPoint(w * a.sin() * r.cos(), w * a.cos() * r.sin())
}

fn julia(&RealPoint(x, y): &RealPoint, w: f64, random_bit: bool) -> RealPoint {
    let a = if random_bit {
        theta(x, y) / 2.0 + PI
    } else {
        theta(x, y) / 2.0
    };
    let r = w * rad2(x, y).powf(0.25);
    RealPoint(r * a.cos(), r * a.sin())
}

fn julia_n<R: Rng + Sized>(
    &RealPoint(x, y): &RealPoint,
    w: f64,
    power: f64,
    dist: f64,
    rng: &mut R,
) -> RealPoint {
    let r_n = power.abs();
    let cn = dist / power / 2.0;

    let a = theta(x, y) + 2.0 * PI * rng.gen_range(0.0..r_n) / power;

    let r = w * (x * x + y * y).powf(cn);
    RealPoint(r * a.cos(), r * a.sin())
}
