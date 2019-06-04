use rayon::prelude::*;
use super::filter::FilterKernel;
use super::filter::gamma_filter;
use super::filter::LogFilter;
use super::filter::spatial_filter;
use super::HDRPixel;
use super::Histogram;

pub struct HistogramProcessor<'a> {
    image_width: u32,
    image_height: u32,
    oversampling: u32,
    spatial_filter: &'a FilterKernel,
    log_filter: LogFilter,
}

impl <'a> HistogramProcessor<'a> {
    pub fn new(quality: u32,
               image_width: u32, image_height: u32,
               view_width: f64, view_height: f64,
               oversampling: u32, brightness: f64,
               spatial_filter: &'a FilterKernel) -> Self {
        let log_filter = LogFilter::new(
            quality,
            oversampling,
            view_width,
            view_height,
            brightness,
        );
        HistogramProcessor { image_width, image_height, oversampling, spatial_filter, log_filter }
    }

    pub fn process_to_raw(&self, histograms: Vec<Histogram>) -> Vec<u8> {
        let histogram: Histogram = HistogramProcessor::combine(histograms);
        self.do_process(histogram)
    }

    fn combine(histograms: Vec<Histogram>) -> Histogram {
        let (width, height) = (histograms[0].width, histograms[0].height);
        let length = (width * height) as usize;

        let data = (0..length).into_par_iter()
            .map(|i| {
                let (mut r, mut b, mut g, mut a) = (0.0, 0.0, 0.0, 0.0);
                for hist in &histograms {
                    add_pixel(&mut r, &mut g, &mut b, &mut a, &hist.data[i]);
                }
                HDRPixel(r, g, b, a as f64)
            }).collect();
        Histogram{data, width, height}
    }

    fn do_process(&self, histogram: Histogram) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];

        let histogram = HistogramProcessor::process_pixels(&self, histogram);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let HDRPixel(r, g, b, _a) = histogram.data[(i + j * self.image_width) as usize];

                result.push((r * 256.0).min(255.0) as u8);
                result.push((g * 256.0).min(255.0) as u8);
                result.push((b * 256.0).min(255.0) as u8);
            }
        }
        result
    }

    fn process_pixels(&self, mut histogram: Histogram) -> Histogram {
        histogram.data = histogram.data.par_iter()
            .map(|pixel| self.log_filter.apply(pixel))
            .collect();

        histogram.data = histogram.data.par_iter()
            .map(|pixel| gamma_filter::apply(&pixel))
            .collect();

        histogram = spatial_filter::apply_filter(
            &self.spatial_filter,
            &histogram,
            self.image_width,
            self.image_height,
            self.oversampling);

        histogram
    }
}

fn add_pixel(r: &mut f64, g: &mut f64, b: &mut f64, a: &mut f64, &HDRPixel(rn, gn, bn, an): &HDRPixel) {
    *r = *r + rn;
    *g = *g + gn;
    *b = *b + bn;
    *a = *a + an;
}


