use std::collections::HashMap;
use super::extract;
use super::extract_all;
use template::builders;
use transforms::Transform;
use variations::Variations;
use variations::Variation;
use variations::VariationType::Linear;

pub fn extract_transform(attributes: &HashMap<String, String>) -> Transform {
    let weight = extract("weight", 1.0, attributes);
    let color = extract("color", 1.0, attributes);

    let coefs = extract_all("coefs", "1.0 0.0 0.0 1.0 0.0 0.0", attributes);
    assert_eq!(6, coefs.len());

    let (a, d, b, e, c, f) = (coefs[0], coefs[1], coefs[2], coefs[3], coefs[4], coefs[5], );
    let var = Variations::new(vec![Variation { variation_type: Linear, weight: 1.0 }]);
    builders::transform(weight, color, [a, b, c, d, e, f], var)
}
