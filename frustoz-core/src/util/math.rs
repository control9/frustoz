use std::iter::Sum;
use std::ops;

pub const EPSILON: f64 = 0.00000001;

pub fn radius(x: f64, y: f64) -> f64 {
    (x.powi(2) + y.powi(2)).sqrt()
}

pub fn rad2(x: f64, y: f64) -> f64 {
    x.powi(2) + y.powi(2)
}

pub fn theta(x: f64, y: f64) -> f64 {
    x.atan2(y)
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct RealPoint(pub f64, pub f64);

impl<'a> ops::Mul<RealPoint> for f64 {
    type Output = RealPoint;

    fn mul(self, RealPoint(x, y): RealPoint) -> RealPoint {
        RealPoint(self * x, self * y)
    }
}

impl Sum<RealPoint> for RealPoint {
    fn sum<I: Iterator<Item = RealPoint>>(iter: I) -> Self {
        let (mut x, mut y) = (0.0, 0.0);
        for RealPoint(x2, y2) in iter {
            x += x2;
            y += y2;
        }
        RealPoint(x, y)
    }
}

#[derive(Debug, PartialEq)]
pub struct ProjectivePoint(pub f64, pub f64, pub f64);

impl<'a> From<&'a RealPoint> for ProjectivePoint {
    fn from(&RealPoint(x, y): &RealPoint) -> Self {
        ProjectivePoint(x, y, 1.0)
    }
}

impl<'a> Into<RealPoint> for &'a ProjectivePoint {
    fn into(self) -> RealPoint {
        let &ProjectivePoint(x, y, z) = self;
        assert_ne!((0.0, 0.0, 0.0), (x, y, z));
        RealPoint(x / z, y / z)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct TransformMatrix(
    pub (f64, f64, f64),
    pub (f64, f64, f64),
    pub (f64, f64, f64),
);

impl ProjectivePoint {
    fn transform(&self, transform: &TransformMatrix) -> Self {
        let &TransformMatrix((a1_1, a1_2, a1_3), (a2_1, a2_2, a2_3), (a3_1, a3_2, a3_3)) =
            transform;
        let &ProjectivePoint(x, y, z) = self;
        ProjectivePoint(
            a1_1 * x + a1_2 * y + a1_3 * z,
            a2_1 * x + a2_2 * y + a2_3 * z,
            a3_1 * x + a3_2 * y + a3_3 * z,
        )
    }
}

impl<'a, 'b> ops::Mul<&'a ProjectivePoint> for &'b TransformMatrix {
    type Output = ProjectivePoint;

    fn mul(self, point: &'a ProjectivePoint) -> ProjectivePoint {
        point.transform(self)
    }
}

#[cfg(test)]
mod projections_test {
    use super::ProjectivePoint;
    use super::RealPoint;

    #[test]
    pub fn should_transform_projective_point_to_real() {
        let non_canonical = &ProjectivePoint(10.0, 15.0, 5.0);
        let real = &RealPoint(2.0, 3.0);
        let canonical = &ProjectivePoint(2.0, 3.0, 1.0);

        let unprojected: &RealPoint = &non_canonical.into();
        assert_eq!(real, unprojected);

        let reprojected: &ProjectivePoint = &unprojected.into();
        assert_eq!(canonical, reprojected);
    }
}
