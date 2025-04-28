use crate::render::FloatPixel;
use crate::util::math::EPSILON;

const GAMMA_FACTOR: f64 = 1.0 / 4.0;

pub fn apply(&FloatPixel(r, g, b, a): &FloatPixel) -> FloatPixel {
    let new_a;
    let gamma_scale;
    if a < EPSILON {
        gamma_scale = 1.0;
        new_a = a;
    } else {
        new_a = apply_gamma(a);
        gamma_scale = new_a / a;
    }

    FloatPixel(r * gamma_scale, g * gamma_scale, b * gamma_scale, new_a)
}

fn apply_gamma(color: f64) -> f64 {
    color.powf(GAMMA_FACTOR).min(1.0).max(0.0)
}
