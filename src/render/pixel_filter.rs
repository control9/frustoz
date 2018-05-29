use std::f64::consts::FRAC_2_PI;
use std::f64::consts::PI;

const GAUSS_SUPPORT: f64 = 1.5;
const HERMITE_SUPPORT: f64 = 1.0;

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

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum FilterType {
    Gaussian,
    Hermite,
}

impl FilterType {
    pub fn apply(&self, x: f64) -> f64 {
        match self {
            &FilterType::Gaussian => gaussian(x),
            &FilterType::Hermite => hermite(x),
        }
    }

    pub fn get_spatial_support(&self) -> f64 {
        match self {
            &FilterType::Gaussian => GAUSS_SUPPORT,
            &FilterType::Hermite => HERMITE_SUPPORT,
        }
    }
}

pub struct PixelFilter {
    pub width: u32,
    pub coefficients: Vec<f64>,
}