use render::HDRPixel;
use super::LogFilter;


impl LogFilter {
    pub fn new(quality: u32, oversampling: u32, width: f64, height: f64, brightness: f64) -> Self {
        let area = width * height; // TODO: FIX to be taken from CameraConfig instead

        let k1 = brightness * 2.0 / oversampling as f64;
        let k2 = 1.0 / (area * quality as f64);
//        let white_level = 240.0;
//        let low_density_brightness = 0.24;
//        let bg_glow = low_density_brightness * k2 * area / self.oversampling as f64;
        LogFilter { k1, k2 }
    }

    pub fn apply(&self, &HDRPixel(r, g, b, a): &HDRPixel) -> HDRPixel {
        let scale = self.get_scale(a as f64);
        HDRPixel(r * scale, g * scale, b * scale, a * scale)
    }

    pub fn get_scale(&self, x: f64) -> f64 {
        if x == 0.0 {
            return 0.0;
        }
        (self.k1 * (1.0 + self.k2 * x as f64).log10()) / (x as f64)
    }
}