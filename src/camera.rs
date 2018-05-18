use coordinates::PlanePoint;
use coordinates::CameraCoordinates;

pub struct Camera {
    origin: PlanePoint,
    scale_x: f64,
    scale_y: f64,
}


impl Camera {
    pub fn new(origin: PlanePoint, scale_x: f64, scale_y: f64) -> Self {
        Self { origin, scale_x, scale_y }
    }

    pub fn project(&self, &PlanePoint(point_x, point_y): &PlanePoint) -> CameraCoordinates {
        let &PlanePoint(origin_x, origin_y) = &self.origin;
        let x = (point_x - origin_x) / self.scale_x;
        let y = (point_y - origin_y) / self.scale_y;
        CameraCoordinates(x,y)
    }
}


#[cfg(test)]
mod camera_test {
    use super::Camera;
    use super::PlanePoint;
    use super::CameraCoordinates;

    #[test]
    fn should_project_identity_square() {
        let origin = PlanePoint( -1.0,  -1.0);
        let camera = Camera::new(
            origin, 2.0, 2.0,
        );

        let point = PlanePoint(-1.0,  -1.0);
        assert_eq!(CameraCoordinates(0.0, 0.0), camera.project(&point));

        let point = PlanePoint(0.0,1.0);
        assert_eq!(CameraCoordinates(0.5, 1.0), camera.project(&point));
    }
}

