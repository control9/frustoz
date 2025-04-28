use super::Camera;
use crate::util::coordinates::CameraCoordinates;
use crate::util::math::RealPoint;

// Camera is a rectangle on RealPoint plane, with left bottom corner at origin point
// an size of (scale_x, scale_y), which has its own coordinate grid, in which
// left bottom corner is (0,0) and right top (1, 1).

// We use it as intermediate step to determine "where in image this RealPoint is,
// an then project it to Canvas, which is integer coordinate grid
// of size (image_width,image_height) over same rectangle.

impl Camera {
    pub fn new(origin: RealPoint, scale_x: f64, scale_y: f64) -> Self {
        Self {
            origin,
            scale_x,
            scale_y,
        }
    }

    pub fn project(&self, &RealPoint(point_x, point_y): &RealPoint) -> CameraCoordinates {
        let &RealPoint(origin_x, origin_y) = &self.origin;
        let x = (point_x - origin_x) / self.scale_x;
        let y = (point_y - origin_y) / self.scale_y;
        CameraCoordinates(x, y)
    }
}

#[cfg(test)]
mod camera_test {
    use super::Camera;
    use super::CameraCoordinates;
    use super::RealPoint;

    #[test]
    fn should_project_identity_square() {
        let origin = RealPoint(-1.0, -1.0);
        let camera = Camera::new(origin, 2.0, 2.0);

        let point = RealPoint(-1.0, -1.0);
        assert_eq!(CameraCoordinates(0.0, 0.0), camera.project(&point));

        let point = RealPoint(0.0, 1.0);
        assert_eq!(CameraCoordinates(0.5, 1.0), camera.project(&point));
    }
}
