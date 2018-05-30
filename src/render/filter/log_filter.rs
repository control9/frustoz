use render::HDRPixel;
use super::LogFilter;


impl LogFilter {
    pub fn apply(&self, &HDRPixel(r, g, b, a) : &HDRPixel) -> HDRPixel {
         let scale = self.scale_calculator.calculate(a as f64);
        HDRPixel(r * scale, g * scale, b * scale, a * scale * self.white_level / 255.0)
    }
}