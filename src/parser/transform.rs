use std::collections::HashMap;
use super::extract;
use super::extract_all;
use template::builders;
use transforms::Transform;
use variations::Variation;
use variations::Variations;
use variations::VariationType::*;

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
    let result = attributes.iter()
        .map(|(name, value)| try_extract_variation(name, value))
        .flat_map(|opt_variation| opt_variation.into_iter())
        .collect();

    Variations::new(result)
}

fn try_extract_variation(name: &str, value: &str) -> Option<Variation> {
    match name.split('#').next().unwrap() {
        "linear" => Some(linear(value)),
        "linear3D" => Some(linear(value)),
        "spiral" => Some(spiral(value)),
        _ => None,
    }
}

fn linear(args: &str) -> Variation {
    Variation { variation_type: Linear, weight: args.parse().unwrap_or(1.0) }
}

fn spiral(args: &str) -> Variation {
    Variation { variation_type: Spiral, weight: args.parse().unwrap_or(1.0) }
}