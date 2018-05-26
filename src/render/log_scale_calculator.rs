pub struct LogScaleCalculator {
    k1: f64,
    k2: f64,
}

impl LogScaleCalculator {
    pub fn new(quality: u32, oversampling: u32) -> Self {
        let brightness = 4.0; // TODO: FIX to be taken from RenderConfig instead
        let area = 12.355 * 6.95; // TODO: FIX to be taken from CameraConfig instead

        let k1 = brightness * 2.0 / oversampling as f64;
        let k2 = 1.0 / (area * quality as f64);
//        let white_level = 240.0;
//        let low_density_brightness = 0.24;
//        let bg_glow = low_density_brightness * k2 * area / self.oversampling as f64;
        LogScaleCalculator { k1, k2 }
    }

    pub fn calculate(&self, x: f64) -> f64 {
        if x == 0.0 {
            return 0.0;
        }
        (self.k1 * (1.0 + self.k2 * x as f64).log10()) / (x as f64)
    }
}