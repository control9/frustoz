use render::canvas::Histogram;
use rayon::prelude::*;
use render::RGBACounter;
use render::log_filter::LogFilter;
use render::log_scale_calculator::LogScaleCalculator;
use render::HDRPixel;
use render::gamma_filter;

pub struct HistogramProcessor {
    width: u32,
    height: u32,
    log_filter: LogFilter,
}

impl HistogramProcessor {
    pub fn new(quality: u32, width: u32, height: u32, oversampling: u32) -> Self {
        let scale_calculator = LogScaleCalculator::new(quality, oversampling);
        let log_filter = LogFilter {
            scale_calculator,
            width,
            height,
            oversampling,
            white_level: 240.0,
        };
        HistogramProcessor { width, height, log_filter }
    }

    pub fn process_to_raw(&self, histograms: Vec<Histogram>) -> Vec<u8> {
        let histogram: Histogram = HistogramProcessor::combine(histograms);
        self.do_process(histogram)
    }

    fn combine(histograms: Vec<Histogram>) -> Histogram {
        let length = histograms.iter()
            .map(|h| h.len())
            .min().unwrap_or(0);

        (0..length).into_par_iter()
            .map(|i| {
                let (mut r, mut b, mut g, mut a) = (0.0, 0.0, 0.0, 0);
                for hist in &histograms {
                    add_pixel(&mut r, &mut g, &mut b, &mut a, &hist[i]);
                }
                RGBACounter(r, g, b, a)
            }).collect()
    }

    fn do_process(&self, histogram: Histogram) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];

        for j in 0..self.height {
            for i in 0..self.width {
                let HDRPixel(r, g, b, _a) = self.extract_pixel(i, j, &histogram);

                result.push((r * 255.0).min(255.0) as u8);
                result.push((g * 255.0).min(255.0) as u8);
                result.push((b * 255.0).min(255.0) as u8);
            }
        }
        result
    }

    fn extract_pixel(&self, x: u32, y: u32, histogram: &Histogram) -> HDRPixel {
        let log_density = self.log_filter.apply(x, y, histogram);
        gamma_filter::apply(&log_density)
    }
}

fn add_pixel(r: &mut f64, g: &mut f64, b: &mut f64, a: &mut u64, &RGBACounter(rn, gn, bn, an): &RGBACounter) {
    *r = *r + rn;
    *g = *g + gn;
    *b = *b + bn;
    *a = *a + an;
}


