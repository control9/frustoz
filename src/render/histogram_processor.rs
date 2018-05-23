use render::canvas::Histogram;
use render::HDRPixel;


const GAMMA_FACTOR: f64 = 1.0 / 2.2;
const EPSILON : f64 = 0.0000000001;

pub struct HistogramProcessor {
    quality: u32,
}

impl HistogramProcessor {
    pub fn new(quality: u32) -> Self {
        HistogramProcessor{quality}
    }

    pub fn process_to_raw(&self, histograms: Vec<Histogram>) -> Vec<u8> {
        let histogram : Histogram = HistogramProcessor::combine(histograms);
        self.do_process(histogram)
    }

    fn combine(histograms: Vec<Histogram>) -> Histogram {
        let length = histograms.iter()
            .map(|h| h.len())
            .min().unwrap_or(0);

        let mut result = vec![HDRPixel(0.0, 0.0, 0.0, 0.0); length];
        for hist in histograms {
            for (i, &pixel) in hist.iter().take(length).enumerate() {
                result[i] = result[i] + pixel;
            }
        }
        result
    }

    fn do_process(&self, histogram: Histogram) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        for HDRPixel(r, g, b, a) in histogram {
            let scale: f64 = 2.5 * (1.0 + a).log10() / self.quality as f64;
            let mut new_a = a * scale;
            let gamma_scale;
            if new_a < EPSILON {
                gamma_scale = 1.0;
            } else {
                let gamma_a = apply_gamma(new_a);
                gamma_scale = gamma_a / new_a;
            }

            result.push((r * scale * gamma_scale) as u8);
            result.push((g * scale * gamma_scale) as u8);
            result.push((b * scale * gamma_scale) as u8);
        }
        result
    }

}
fn apply_gamma(color: f64) -> f64 {
    (color / 255.0).min(1.0).max(0.0).powf(GAMMA_FACTOR) * 255.0
}

