use math::TransformMatrix;
use math::RealPoint;
use math::ProjectivePoint;

type TM = TransformMatrix;

pub struct Transformation {
    transforms: Vec<TM>,
    relative_weights: Vec<f64>,
}

impl Transformation {
    pub fn new(weighted_transforms: Vec<(TM, f64)>) -> Self {
        let total_weight: f64 = weighted_transforms.iter()
            .map(|&(_, weight)| weight)
            .sum();

        let mut transforms: Vec<TransformMatrix> = vec![];
        let mut relative_weights = vec![];

        for (t, w) in weighted_transforms {
            transforms.push(t);
            relative_weights.push(w / total_weight);
        }

        Self { transforms, relative_weights }
    }

    fn get_transformation(&self, seed: f64) -> &TM {
        assert!(seed >= 0.0, "seed should in [0, 1) range");
        let mut accumulated_weight = 0.0;
        for i in 0..self.transforms.len() {
            accumulated_weight += self.relative_weights[i];
            if accumulated_weight > seed {
                return &self.transforms[i];
            }
        }
        panic!("Seed is greater than 1 or incorrect Transformation")
    }

    pub fn apply_transform(&self, seed: f64, point: &RealPoint) -> RealPoint {
        let pr: ProjectivePoint = point.into();

        let transform: &TransformMatrix = self.get_transformation(seed);
        let result_pr = &(transform * &pr);
        let result: RealPoint = result_pr.into();
        result
    }
}