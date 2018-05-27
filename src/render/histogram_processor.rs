use rayon::prelude::*;
use render::canvas::HistogramLayer;
use render::gamma_filter;
use render::HDRPixel;
use render::Histogram;
use render::log_filter::LogFilter;
use render::log_scale_calculator::LogScaleCalculator;
use render::RGBACounter;
use render::spatial_filter;

pub struct HistogramProcessor {
    width: u32,
    height: u32,
    oversampling: u32,
    spatial_filter: (usize, Vec<f64>),
    log_filter: LogFilter,
}

impl HistogramProcessor {
    pub fn new(quality: u32, width: u32, height: u32, oversampling: u32, spatial_filter: (usize, Vec<f64>)) -> Self {
        let scale_calculator = LogScaleCalculator::new(quality, oversampling);
        let log_filter = LogFilter {
            scale_calculator,
            width,
            height,
            oversampling,
            white_level: 240.0,
        };
        HistogramProcessor { width, height, oversampling, spatial_filter, log_filter }
    }

    pub fn process_to_raw(&self, histograms: Vec<HistogramLayer>) -> Vec<u8> {
        let histogram: Histogram = HistogramProcessor::combine(histograms);
        self.do_process(histogram)
    }

    fn combine(histograms: Vec<HistogramLayer>) -> Histogram {
        let length = histograms.iter()
            .map(|h| h.len())
            .min().unwrap_or(0);

        (0..length).into_par_iter()
            .map(|i| {
                let (mut r, mut b, mut g, mut a) = (0.0, 0.0, 0.0, 0);
                for hist in &histograms {
                    add_pixel(&mut r, &mut g, &mut b, &mut a, &hist[i]);
                }
                HDRPixel(r, g, b, a as f64)
            }).collect()
    }

    fn do_process(&self, histogram: Histogram) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];

        let histogram = HistogramProcessor::process_pixels(&self, histogram);

        for j in 0..self.height {
            for i in 0..self.width {
                let HDRPixel(r, g, b, _a) = histogram[(i + j * self.width) as usize];

                result.push((r * 255.0).min(255.0) as u8);
                result.push((g * 255.0).min(255.0) as u8);
                result.push((b * 255.0).min(255.0) as u8);
            }
        }
        result
    }

    fn process_pixels(&self, histogram: Histogram) -> Histogram {
        let border = (self.spatial_filter.0 - self.oversampling as usize).max(0) as u32;
        let hist = spatial_filter::apply_filter(
            (&self.spatial_filter.0, &self.spatial_filter.1),
            &histogram,
            self.width,
            self.height,
            self.width * self.oversampling + border * 2,
            self.height * self.oversampling + border * 2,
            self.oversampling);
        hist.iter()
            .map(|pixel| self.log_filter.apply(pixel))
            .map(|pixel| gamma_filter::apply(&pixel))
            .collect()
    }
}

fn add_pixel(r: &mut f64, g: &mut f64, b: &mut f64, a: &mut u64, &RGBACounter(rn, gn, bn, an): &RGBACounter) {
    *r = *r + rn;
    *g = *g + gn;
    *b = *b + bn;
    *a = *a + an;
}


