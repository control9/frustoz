use util::coordinates::CameraCoordinates;
use util::coordinates::CanvasPixel;
use template::palette::RGB;
use std::f64;
use render::RGBACounter;

pub type Histogram = Vec<RGBACounter>;

pub struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<RGBACounter>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height, pixels: vec![RGBACounter(0.0, 0.0, 0.0, 0); (width * height) as usize] }
    }

    fn project(&self, coordinates: &CameraCoordinates) -> Option<CanvasPixel> {
        if !valid_coordinates(coordinates) {
            return None;
        }
        let &CameraCoordinates(x, y) = coordinates;
        Some(CanvasPixel(
            (self.width as f64 * x) as u32,
            (self.height as f64 * y) as u32,
        ))
    }

    pub fn project_and_update(&mut self, coordinates: &CameraCoordinates, color: &RGB) {
        let pixel = self.project(coordinates);
        match pixel {
            Some(p) => self.update(p, color),
            None => ()
        }
    }

    fn update(&mut self, CanvasPixel(x, y): CanvasPixel, color: &RGB) {
        let pixel_index: usize = (y * self.width + x) as usize;
        self.update_pixel(pixel_index, color);
    }

    fn update_pixel(&mut self, index: usize, &RGB(r, g, b): &RGB) {
        let &RGBACounter(rc, gc, bc, a) = &self.pixels[index];
        self.pixels[index] = RGBACounter(rc + r as f64, gc + g as f64, bc + b as f64, a + 1);
    }

    pub fn histogram(self) -> Histogram {
        self.pixels
    }
}

fn valid_coordinates(&CameraCoordinates(x, y): &CameraCoordinates) -> bool {
    0.0 <= x && x < 1.0 && 0.0 <= y && y < 1.0
}

#[cfg(test)]
mod canvas_test {
    use super::Canvas;
    use super::CameraCoordinates;
    use super::CanvasPixel;
    use std::f64;

    #[test]
    pub fn should_accept_camera_coordinates_between_zero_and_one() {
        assert!(super::valid_coordinates(&CameraCoordinates(0.5, 0.5)));
        assert!(super::valid_coordinates(&CameraCoordinates(0.0, 0.0)));
        assert!(super::valid_coordinates(&CameraCoordinates(0.9999, 0.9999)));

        assert!(!super::valid_coordinates(&CameraCoordinates(-0.1, 0.0)));
        assert!(!super::valid_coordinates(&CameraCoordinates(0.0, 1.0)));
        assert!(!super::valid_coordinates(&CameraCoordinates(f64::NAN, 0.0)));
    }

    #[test]
    pub fn should_map_left_upper_corner() {
        should_project_coordinates(CanvasPixel(0, 0), CanvasSize(8, 5), CameraCoordinates(0.0, 0.0));
    }

    #[test]
    pub fn should_map_different_coordinates_correctly() {
        should_project_coordinates(CanvasPixel(3, 3), CanvasSize(8, 5), CameraCoordinates(0.49, 0.79));
    }

    #[test]
    pub fn should_map_point_near_right_down_corner() {
        should_project_coordinates(CanvasPixel(7, 4), CanvasSize(8, 5), CameraCoordinates(0.99, 0.99));
    }

    #[test]
    pub fn should_not_map_when_one_of_coordinates_higher_than_one() {
        should_not_project_coordinates(CanvasSize(8, 5), CameraCoordinates(1.3, 0.4));
        should_not_project_coordinates(CanvasSize(8, 5), CameraCoordinates(0.4, 1.3));
    }

    #[test]
    pub fn should_not_map_when_one_of_coordinates_less_than_zero() {
        should_not_project_coordinates(CanvasSize(8, 5), CameraCoordinates(-0.3, 0.5));
        should_not_project_coordinates(CanvasSize(8, 5), CameraCoordinates(0.5, -0.3));
    }

    #[test]
    pub fn should_not_map_when_point_is_on_right_or_down_border() {
        should_not_project_coordinates(CanvasSize(8, 5), CameraCoordinates(1.0, 0.5));
        should_not_project_coordinates(CanvasSize(8, 5), CameraCoordinates(0.5, 1.0));
    }

    struct CanvasSize(u32, u32);

    fn should_project_coordinates(expected: CanvasPixel, CanvasSize(width, height): CanvasSize, coordinates: CameraCoordinates) {
        let canvas = &Canvas::new(width, height);
        let actual = canvas.project(&coordinates);

        assert_eq!(Some(expected), actual);
    }

    fn should_not_project_coordinates(CanvasSize(width, height): CanvasSize, coordinates: CameraCoordinates) {
        let canvas = &Canvas::new(width, height);
        let actual = canvas.project(&coordinates);
        assert!(actual.is_none());
    }
}
