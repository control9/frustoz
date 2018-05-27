use render::log_scale_calculator::LogScaleCalculator;
use render::canvas::HistogramLayer;
use render::HDRPixel;
use render::RGBACounter;

pub struct LogFilter {
    pub scale_calculator: LogScaleCalculator,
    pub width: u32,
    pub height: u32,
    pub oversampling: u32,
    pub white_level: f64,
}


impl LogFilter {
    pub fn apply(&self, &HDRPixel(r, g, b, a) : &HDRPixel) -> HDRPixel {
         let scale = self.scale_calculator.calculate(a as f64);
        HDRPixel(r * scale, g * scale, b * scale, a * scale * self.white_level / 255.0)
    }
}