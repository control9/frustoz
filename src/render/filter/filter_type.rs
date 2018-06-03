use std::f64::consts::FRAC_2_PI;
use std::f64::consts::PI;
use super::FilterType;

const GAUSS_SUPPORT: f64 = 1.5;
const HERMITE_SUPPORT: f64 = 1.0;
const BOX_SUPPORT: f64 = 0.5;
const TRIANGLE_SUPPORT: f64 = 1.0;
const MITCHELL_SUPPORT: f64 = 2.0;


const MITCHELL_B: f64 = 1.0/3.0;
const MITCHELL_C: f64 = 1.0/3.0;


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

fn mitchell(x: f64) -> f64 {
    let mut t = x.abs();
    let tt = t * t;
    let ttt = t*t*t;

    match t {
        _ if t < 1.0 => {
            t = ttt * (12.0 - 9.0 * MITCHELL_B - 6.0 * MITCHELL_C) +
                tt * (-18.0 + 12.0 * MITCHELL_B + 6.0 * MITCHELL_C) +
                (6.0 - 2.0 * MITCHELL_B);
            t / 6.0
        }
        _ if t < 2.0 => {
            t = ttt * (-1.0 * MITCHELL_B - 6.0 * MITCHELL_C) +
                tt * (6.0 * MITCHELL_B + 30.0 * MITCHELL_C) +
                t * (-12.0 * MITCHELL_B - 48.0 * MITCHELL_C) +
                (8.0 * MITCHELL_B + 24.0 * MITCHELL_C);
            t / 6.0
        }
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
            &FilterType::Mitchell => mitchell(x),
        }
    }

    pub fn get_spatial_support(&self) -> f64 {
        match self {
            &FilterType::Gaussian => GAUSS_SUPPORT,
            &FilterType::Hermite => HERMITE_SUPPORT,
            &FilterType::Box => BOX_SUPPORT,
            &FilterType::Triangle => TRIANGLE_SUPPORT,
            &FilterType::Mitchell => MITCHELL_SUPPORT,
        }
    }
}
