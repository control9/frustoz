use render::log_scale_calculator::LogScaleCalculator;
use render::canvas::Histogram;
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
    pub fn apply(&self, x: u32, y: u32, histogram: &Histogram) -> HDRPixel {
        let os = self.oversampling;
        let (mut r, mut b, mut g, mut a) = (0.0, 0.0, 0.0, 0.0);
        for j in 0..self.oversampling {
            for i in 0..self.oversampling {
                let index = (x * os + i) + (y * os + j) * &self.width * os;
                let &RGBACounter(rp, gp, bp, ap) = &histogram[index as usize];

                let scale = self.scale_calculator.calculate(ap as f64);

                r += rp * scale;
                g += gp * scale;
                b += bp * scale;
                a += ap as f64 * scale * (self.white_level / 255.0);
            }
        }

        HDRPixel(r, g, b, a)
    }
}