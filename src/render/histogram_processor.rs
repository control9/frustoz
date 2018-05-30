use rayon::prelude::*;
use render::filter::FilterKernel;
use render::filter::LogFilter;
use render::filter::LogScaleCalculator;
use render::filter::spatial_filter;
use render::filter::gamma_filter;
use render::HDRPixel;
use render::Histogram;
use render::histogram::canvas::HistogramLayer;
use render::RGBACounter;

pub struct HistogramProcessor {
    image_width: u32,
    image_height: u32,
    histogram_width: u32,
    histogram_height: u32,
    oversampling: u32,
    spatial_filter: FilterKernel,
    log_filter: LogFilter,
}

impl HistogramProcessor {
    pub fn new(quality: u32, image_width: u32, image_height: u32,
               histogram_width: u32, histogram_height: u32,
               oversampling: u32, spatial_filter: FilterKernel) -> Self {
        let scale_calculator = LogScaleCalculator::new(quality, oversampling);
        let log_filter = LogFilter {
            scale_calculator,
            width: image_width,
            height: image_height,
            oversampling,
            white_level: 240.0,
        };
        HistogramProcessor { image_width, image_height, histogram_width, histogram_height, oversampling, spatial_filter, log_filter }
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

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let HDRPixel(r, g, b, _a) = histogram[(i + j * self.image_width) as usize];

                result.push((r * 255.0).min(255.0) as u8);
                result.push((g * 255.0).min(255.0) as u8);
                result.push((b * 255.0).min(255.0) as u8);
            }
        }
        result
    }

    fn process_pixels(&self, histogram: Histogram) -> Histogram {
        let hist = spatial_filter::apply_filter(
            &self.spatial_filter,
            &histogram,
            self.image_width,
            self.image_height,
            self.histogram_width,
            self.histogram_height,
            self.oversampling);
        hist.par_iter()
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


