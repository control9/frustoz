use std::ops::Deref;

use super::filter::gamma_filter;
use super::filter::spatial_filter;
use super::filter::FilterKernel;
use super::filter::LogFilter;
use super::{Canvas, NormalizedCanvas};
use crate::render::FloatPixel;
use rayon::prelude::*;

#[derive(Clone)]
pub struct CanvasProcessor {
    image_width: u32,
    image_height: u32,
    oversampling: u32,
    spatial_filter: FilterKernel,
    log_filter: LogFilter,
}

impl CanvasProcessor {
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
        CanvasProcessor {
            image_width,
            image_height,
            oversampling,
            spatial_filter,
            log_filter,
        }
    }

    pub fn process_to_raw_single<T>(&self, canvas: T) -> Vec<u8>
    where
        T: Deref<Target = Canvas> + Sync,
    {
        let preprocessed_canvas: NormalizedCanvas = Self::preprocess(canvas);
        let processed_canvas = Self::apply_filters(&self, preprocessed_canvas);

        let mut result: Vec<u8> = vec![];
        result.reserve( (self.image_height * self.image_width * 3) as usize);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let FloatPixel(r, g, b, _a) = processed_canvas.data[(i + j * self.image_width) as usize];

                result.push((r * 256.0).min(255.0) as u8);
                result.push((g * 256.0).min(255.0) as u8);
                result.push((b * 256.0).min(255.0) as u8);
            }
        }
        result
    }
    

    fn preprocess<T>(canvas: T) -> NormalizedCanvas
    where
        T: Deref<Target = Canvas> + Sync,
    {
        let (width, height) = (canvas.width, canvas.height);
        let length = (width * height) as usize;

        let data = (0..length)
            .into_par_iter()
            .map(|i| {
                FloatPixel::from_hdr_pixel(&canvas.data[i])
            })
            .collect();
        NormalizedCanvas {
            data,
            width,
            height,
        }
    }

    fn apply_filters(&self, mut canvas: NormalizedCanvas) -> NormalizedCanvas {
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
