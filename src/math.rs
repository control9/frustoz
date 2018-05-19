#[derive(Debug, PartialEq)]
pub struct RealPoint(pub f64, pub f64);

#[derive(Debug, PartialEq)]
pub struct ProjectivePoint(pub f64, pub f64, pub f64);

impl <'a> From<&'a RealPoint> for ProjectivePoint {
    fn from(&RealPoint(x, y): &RealPoint) -> Self {
        ProjectivePoint(x, y, 1.0)
    }
}


impl <'a> Into<RealPoint> for &'a ProjectivePoint {
    fn into(self) -> RealPoint {
        let &ProjectivePoint(x, y, z) = self;
        assert_ne!((0.0, 0.0, 0.0), (x, y, z));
        RealPoint(x / z, y / z)
    }
}

#[derive(Debug, PartialEq)]
pub struct TransformMatrix(
    pub (f64, f64, f64),
    pub (f64, f64, f64),
    pub (f64, f64, f64),
);


impl ProjectivePoint {
    fn transform(&self, transform: &TransformMatrix) -> Self {
        let &TransformMatrix(
            (a1_1, a1_2, a1_3),
            (a2_1, a2_2, a2_3),
            (a3_1, a3_2, a3_3),
        ) = transform;
        let &ProjectivePoint(x, y, z) = self;
        ProjectivePoint(
            a1_1 * x + a1_2 * y + a1_3 * z,
            a2_1 * x + a2_2 * y + a2_3 * z,
            a3_1 * x + a3_2 * y + a3_3 * z,
        )
    }
}
use std::ops;

impl <'a, 'b> ops::Mul<&'a ProjectivePoint> for &'b TransformMatrix {
    type Output = ProjectivePoint;

    fn mul(self, point: &'a ProjectivePoint) -> ProjectivePoint {
        point.transform(self)
    }
}

#[cfg(test)]
mod projections_test {
    use super::RealPoint;
    use super::ProjectivePoint;

    #[test]
    pub fn should_transform_projective_point_to_real() {
        let non_canonical = &ProjectivePoint(10.0, 15.0, 5.0);
        let real = &RealPoint(2.0, 3.0);
        let canonical = &ProjectivePoint(2.0, 3.0, 1.0);

        let unprojected : &RealPoint = &non_canonical.into();
        assert_eq!(real, unprojected);

        let reprojected : &ProjectivePoint = &unprojected.into();
        assert_eq!(canonical, reprojected);
    }
}