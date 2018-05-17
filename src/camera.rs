use std::f64;

#[derive(Debug, PartialEq)]
pub struct RealPoint {
    x: f64,
    y: f64,
}

#[derive(Debug, Eq, PartialEq)]
pub struct IntPoint {
    x: i32,
    y: i32,
}

pub struct Camera {
    pixel_amount: i32,

    render_width: i32,
    render_height: i32,

    camera_from_x: f64,
    camera_from_y: f64,
    camera_to_x: f64,
    camera_to_y: f64,

    pixels: Vec<i64>,
}

impl Camera {
    pub fn new(render_width: i32,
               render_height: i32,
               camera_from_x: f64,
               camera_from_y: f64,
               camera_to_x: f64,
               camera_to_y: f64) -> Self {
        let size = render_width * render_height;
        Self {
            pixel_amount: size,

            render_width,
            render_height,

            camera_from_x,
            camera_from_y,
            camera_to_x,
            camera_to_y,

            pixels: vec![0i64; size as usize],
        }
    }

    pub fn inside(&self, point: &RealPoint) -> bool {
        match (point.x, point.y) {
            (x, y) if (x.is_nan() || y.is_nan()) => false,
            (x, y) if x >= self.camera_to_x => false,
            (x, y) if y >= self.camera_to_y => false,
            (x, y) if x < self.camera_from_x => false,
            (x, y) if y < self.camera_from_x => false,
            _ => true
        }
    }

    pub fn project(&self, point: &RealPoint) -> IntPoint {
        if !self.inside(point) {
            panic!("Point outside frame")
        }
        let width = self.camera_to_x - self.camera_from_x;
        let wperc = (point.x - self.camera_from_x) / width;

        let height = self.camera_to_y - self.camera_from_y;
        let hperc = (point.y - self.camera_from_y) / height;

        IntPoint {
            x: (self.render_width as f64 * wperc) as i32,
            y: (self.render_height as f64 * hperc) as i32,
        }
    }
}


#[cfg(test)]
mod test {
    use super::Camera;
    use super::RealPoint;
    use super::IntPoint;

    #[test]
    fn basics() {
        let camera = Camera::new(
            100, 100,
            -1f64, -1f64, 1f64, 1f64,
        );

        let point = RealPoint {
            x: 0.99999,
            y: -0.9999,
        };

        assert!(camera.inside(&point));
        assert_eq!(IntPoint { x: 99, y: 0 }, camera.project(&point));

        let point = RealPoint {
            x: 2.0,
            y: -0.3,
        };
        assert!(!camera.inside(&point));
    }
}



