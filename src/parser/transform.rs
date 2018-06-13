use std::collections::HashMap;
use super::extract;
use super::extract_all;
use template::builders;
use transforms::Transform;
use variations::Variation;
use variations::Variation::*;
use variations::Variations;

pub fn extract_transform(attributes: &HashMap<String, String>) -> Transform {
    let weight = extract("weight", 1.0, attributes);
    let color = extract("color", 1.0, attributes);

    let coefs = extract_all("coefs", "1.0 0.0 0.0 1.0 0.0 0.0", attributes);
    assert_eq!(6, coefs.len());

    let (a, d, b, e, c, f) = (coefs[0], coefs[1], coefs[2], coefs[3], coefs[4], coefs[5], );
    let var = extract_variations(attributes);
    builders::transform(weight, color, [a, b, c, d, e, f], var)
}

fn extract_variations(attributes: &HashMap<String, String>) -> Variations {
    let result: Vec<Variation> = attributes.iter()
        .map(|(name, value)| try_extract_variation(name, value))
        .flat_map(|opt_variation| opt_variation.into_iter())
        .collect();
    if !result.is_empty() {
        Variations::new(result)
    } else {
        warn!("No transformation type found, assuming linear: {:?}", attributes);
        Variations::new(vec![Linear(1.0)])
    }
}

fn try_extract_variation(name: &str, value: &str) -> Option<Variation> {
    match name.split('#').next().unwrap() {
        "linear" => Some(Linear(value.parse().unwrap_or(1.0))),
        "linear3D" => Some(Linear(value.parse().unwrap_or(1.0))),
        "sinusoidal" => Some(Sinusoidal(value.parse().unwrap_or(1.0))),
        "spherical" => Some(Spherical(value.parse().unwrap_or(1.0))),
        "swirl" => Some(Swirl(value.parse().unwrap_or(1.0))),
        "horseshoe" => Some(Horseshoe(value.parse().unwrap_or(1.0))),
        "polar" => Some(Polar(value.parse().unwrap_or(1.0))),
        "handkerchief" => Some(Handkerchief(value.parse().unwrap_or(1.0))),
        "heart" => Some(Heart(value.parse().unwrap_or(1.0))),
        "disc" => Some(Disc(value.parse().unwrap_or(1.0))),
        "spiral" => Some(Spiral(value.parse().unwrap_or(1.0))),
        "hyperbolic" => Some(Hyperbolic(value.parse().unwrap_or(1.0))),
        "diamond" => Some(Diamond(value.parse().unwrap_or(1.0))),
        _ => None,
    }
}
