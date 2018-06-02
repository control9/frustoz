use std::f64::consts::FRAC_2_PI;
use std::f64::consts::PI;
use super::FilterType;

const GAUSS_SUPPORT: f64 = 1.5;
const HERMITE_SUPPORT: f64 = 1.0;
const BOX_SUPPORT: f64 = 0.5;
const TRIANGLE_SUPPORT: f64 = 1.0;

fn _sinc(x: f64) -> f64 {
    let xt = x * PI;
    match xt {
        zero if zero < 0.00000001 => 1.0,
        _non_zero => xt.sin() / xt
    }
}

fn gaussian(x: f64) -> f64 {
    (-2.0 * x * x).exp() * FRAC_2_PI.sqrt()
}


/// f(x) = 2|x|^3 - 3t^2 + 1, -1 <= t <= 1
fn hermite(x: f64) -> f64 {
    match x {
        positive if 1.0 > positive && positive >= 0.0 =>
            (2.0 * x - 3.0) * x * x + 1.0,
        negative if -1.0 < negative && negative < 0.0 =>
            (-2.0 * x - 3.0) * x * x + 1.0,
        _ => 0.0,
    }
}

fn boxed(x: f64) -> f64 {
    match x {
        _ if x.abs() < 0.5 => 1.0,
        _ => 0.0
    }
}

fn triangle(x: f64) -> f64 {
    match x.abs() {
        t if t < 1.0 => 1.0 - t,
        _ => 0.0
    }
}
impl FilterType {
    pub fn apply(&self, x: f64) -> f64 {
        match self {
            &FilterType::Gaussian => gaussian(x),
            &FilterType::Hermite => hermite(x),
            &FilterType::Box => boxed(x),
            &FilterType::Triangle => triangle(x),
        }
    }

    pub fn get_spatial_support(&self) -> f64 {
        match self {
            &FilterType::Gaussian => GAUSS_SUPPORT,
            &FilterType::Hermite => HERMITE_SUPPORT,
            &FilterType::Box => BOX_SUPPORT,
            &FilterType::Triangle => TRIANGLE_SUPPORT,
        }
    }
}
