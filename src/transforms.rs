use math::TransformMatrix;
use math::RealPoint;
use math::ProjectivePoint;

struct Transform {
    affine: TransformMatrix,
    weight: f64,
    color: f64,
}


pub struct TransformSystem(Vec<Transform>);

impl TransformSystem {
    pub fn new(weighted_transforms: Vec<(TransformMatrix, f64, f64)>) -> Self {
        let total_weight: f64 = weighted_transforms.iter()
            .map(|&(_, weight, _)| weight)
            .sum();

        let mut transforms: Vec<Transform> = vec![];

        for (t, w, c) in weighted_transforms {
            transforms.push(
                Transform { affine: t, weight: w / total_weight, color: c }
            );
        }

        TransformSystem(transforms)
    }

    fn get_transformation(&self, seed: f64) -> &Transform {
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

        let transform: &Transform = self.get_transformation(seed);
        let result_pr = &(&transform.affine * pr);
        let result: RealPoint = result_pr.into();
        result
    }
}