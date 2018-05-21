use util::math::TransformMatrix;
use util::math::RealPoint;
use util::math::ProjectivePoint;

pub struct WeightedTransform {
    affine: TransformMatrix,
    weight: f64,
    color: f64,
}

impl WeightedTransform {
    pub fn apply(&self, point: &RealPoint, color: f64) -> (RealPoint, f64) {
        let pr: &ProjectivePoint = &point.into();
        let result_pr = &(&self.affine * pr);
        let result: RealPoint = result_pr.into();
        let result_color = (color + self.color) / 2.0;
        (result, result_color)
    }
}

pub struct TransformSystem(Vec<WeightedTransform>);

impl TransformSystem {
    pub fn new(weighted_transforms: Vec<(TransformMatrix, f64, f64)>) -> Self {
        let total_weight: f64 = weighted_transforms.iter()
            .map(|&(_, weight, _)| weight)
            .sum();

        let mut transforms: Vec<WeightedTransform> = vec![];

        for (t, w, c) in weighted_transforms {
            transforms.push(
                WeightedTransform { affine: t, weight: w / total_weight, color: c }
            );
        }

        TransformSystem(transforms)
    }

    pub fn get_transformation(&self, seed: f64) -> &WeightedTransform {
        assert!(seed >= 0.0, "seed should in [0, 1) range");
        let mut accumulated_weight = 0.0;
        for i in 0..self.0.len() {
            accumulated_weight += self.0[i].weight;
            if accumulated_weight > seed {
                return &self.0[i];
            }
        }
        panic!("Seed is greater than 1 or incorrect Transformation")
    }

    pub fn apply_transform(&self, seed: f64, point: &RealPoint) -> RealPoint {
        let pr: &ProjectivePoint = &point.into();

        let transform: &WeightedTransform = self.get_transformation(seed);
        let result_pr = &(&transform.affine * pr);
        let result: RealPoint = result_pr.into();
        result
    }
}