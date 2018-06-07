use util::math::TransformMatrix;
use util::math::RealPoint;
use util::math::ProjectivePoint;
use variations::Variations;

#[derive(Clone, Debug)]
pub struct Transform {
    pub affine: TransformMatrix,
    pub variations: Variations,
    pub weight: f64,
    pub color: f64,
}

impl Transform {
    pub fn apply(&self, point: &RealPoint, color: f64) -> (RealPoint, f64) {
        let pr: &ProjectivePoint = &point.into();
        let result_pr = &(&self.affine * pr);
        let result: RealPoint = result_pr.into();
        let result_color = (color + self.color) / 2.0;
        (result, result_color)
    }
}

#[derive(Clone, Debug)]
pub struct TransformSystem {
    transforms: Vec<Transform>,
    total_weight: f64,
}

impl TransformSystem {
    pub fn new(transforms: Vec<Transform>) -> Self {
        let total_weight: f64 = transforms.iter()
            .map(|&Transform{weight, ..}| weight)
            .sum();

        TransformSystem{transforms, total_weight}
    }

    pub fn get_transformation(&self, seed: f64) -> &Transform {
        assert!(seed >= 0.0, "seed should in [0, 1) range");
        let scaled_seed = seed * self.total_weight;
        let mut accumulated_weight = 0.0;
        for transform in self.transforms.iter() {
            accumulated_weight += transform.weight;
            if accumulated_weight > scaled_seed {
                return transform;
            }
        }
        panic!("Seed is greater than 1 or incorrect Transformation")
    }

    pub fn apply_transform(&self, transform_seed: f64, variation_seed: f64, point: &RealPoint) -> RealPoint {
        let pr: &ProjectivePoint = &point.into();

        let transform: &Transform = self.get_transformation(transform_seed);
        let affine_result_pr = &(&transform.affine * pr);
        let affine_result: RealPoint = affine_result_pr.into();
        let result = transform.variations.get_variation(variation_seed).apply(affine_result);
        result
    }
}