use crate::render::FloatPixel;
use std::sync::atomic::Ordering::Relaxed;
use super::filter::gamma_filter;
use super::filter::spatial_filter;
use super::filter::FilterKernel;
use super::filter::LogFilter;
use super::{Canvas, CombinedCanvas, HDRPixel};
use rayon::prelude::*;

#[derive(Clone)]
pub struct CanvasCombiner {
    image_width: u32,
    image_height: u32,
    oversampling: u32,
    spatial_filter: FilterKernel,
    log_filter: LogFilter,
}

impl CanvasCombiner {
    pub fn new(
        quality: u32,
        image_width: u32,
        image_height: u32,
        view_width: f64,
        view_height: f64,
        oversampling: u32,
        brightness: f64,
        spatial_filter: FilterKernel,
    ) -> Self {
        let log_filter = LogFilter::new(quality, oversampling, view_width, view_height, brightness);
        CanvasCombiner {
            image_width,
            image_height,
            oversampling,
            spatial_filter,
            log_filter,
        }
    }

    pub fn process_to_raw(&self, canvases: Vec<Canvas>) -> Vec<u8> {
        let combined_canvas: CombinedCanvas = CanvasCombiner::combine(canvases);
        self.do_process(combined_canvas)
    }

    fn combine(histograms: Vec<Canvas>) -> CombinedCanvas {
        let (width, height) = (histograms[0].width, histograms[0].height);
        let length = (width * height) as usize;

        let data = (0..length)
            .into_par_iter()
            .map(|i| {
                let (mut r, mut b, mut g, mut a) = (0.0, 0.0, 0.0, 0.0);
                for hist in &histograms {
                    add_pixel(&mut r, &mut g, &mut b, &mut a, &hist.data[i]);
                }
                FloatPixel(r / 256.0, g/ 256.0, b/ 256.0, a as f64)
            })
            .collect();
        CombinedCanvas {
            data,
            width,
            height,
        }
    }

    fn do_process(&self, histogram: CombinedCanvas) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];

        let histogram = CanvasCombiner::process_pixels(&self, histogram);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let FloatPixel(r, g, b, _a) = histogram.data[(i + j * self.image_width) as usize];

                result.push((r * 256.0).min(255.0) as u8);
                result.push((g * 256.0).min(255.0) as u8);
                result.push((b * 256.0).min(255.0) as u8);
            }
        }
        result
    }

    fn process_pixels(&self, mut canvas: CombinedCanvas) -> CombinedCanvas {
        canvas.data = canvas
            .data
            .par_iter()
            .map(|pixel| self.log_filter.apply(pixel))
            .collect();

        canvas.data = canvas
            .data
            .par_iter()
            .map(|pixel| gamma_filter::apply(&pixel))
            .collect();

        canvas = spatial_filter::apply_filter(
            &self.spatial_filter,
            &canvas,
            self.image_width,
            self.image_height,
            self.oversampling,
        );

        canvas
    }
}

fn add_pixel(
    r: &mut f64,
    g: &mut f64,
    b: &mut f64,
    a: &mut f64,
    pixel: &HDRPixel,
) {
    *r = *r + pixel.0.load(Relaxed) as f64;
    *g = *g + pixel.1.load(Relaxed) as f64;
    *b = *b + pixel.2.load(Relaxed) as f64;
    *a = *a + pixel.3.load(Relaxed) as f64;
}
