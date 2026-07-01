use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;
use crate::model::palette::RGB;

pub mod canvas;
pub mod canvas_processor;
pub mod filter;
pub mod multithreaded_renderer;
mod progressive_render_task;
pub mod progressive_renderer;
pub mod split_render_task;
pub mod tokio_multithreaded_renderer;

#[derive(Clone)]
pub struct Canvas {
    data: Vec<HDRPixel>,
    width: u32,
    height: u32,
}
pub struct HDRPixel(pub AtomicU32, pub AtomicU32, pub AtomicU32, pub AtomicU32);

impl HDRPixel {
    fn zero() -> Self {
        HDRPixel(
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
        )
    }

    fn add(&self, color: &RGB) {
        _ = self.0.fetch_add(color.0.into(), Relaxed);
        _ = self.1.fetch_add(color.1.into(), Relaxed);
        _ = self.2.fetch_add(color.2.into(), Relaxed);
        _ = self.3.fetch_add(1, Relaxed);
    }
}

impl Default for HDRPixel {
    fn default() -> Self {
        Self::zero()
    }
}

impl Clone for HDRPixel {
    fn clone(&self) -> Self {
        HDRPixel(
            AtomicU32::new(*&self.0.load(Relaxed)),
            AtomicU32::new(*&self.1.load(Relaxed)),
            AtomicU32::new(*&self.2.load(Relaxed)),
            AtomicU32::new(*&self.3.load(Relaxed)),
        )
    }
}

#[derive(Clone)]
pub struct NormalizedCanvas {
    data: Vec<FloatPixel>,
    width: u32,
    #[allow(unused)]
    height: u32,
}

#[derive(Copy, Clone)]
pub struct FloatPixel(pub f64, pub f64, pub f64, pub f64);

impl FloatPixel {
    fn from_hdr_pixel(pixel: &HDRPixel) -> Self {
        FloatPixel(
            pixel.0.load(Relaxed) as f64 / 256.0,
            pixel.1.load(Relaxed) as f64 / 256.0,
            pixel.2.load(Relaxed) as f64 / 256.0,
            pixel.3.load(Relaxed) as f64,
        )
    }
}

#[derive(Copy, Clone)]
pub struct Progress(pub u64, pub usize);

pub trait ProgressReporter {
    fn new(iterations_per_thread: &Vec<u64>) -> Self;
    fn report(&mut self, progress: Progress);
}

#[derive(Copy, Clone)]
pub struct NoOpReporter {}

impl ProgressReporter for NoOpReporter {
    fn new(_: &Vec<u64>) -> Self {
        NoOpReporter {}
    }

    fn report(&mut self, _: Progress) {}
}

fn split(iterations: u64, parts: u32) -> Vec<u64> {
    if parts == 1 {
        return vec![iterations];
    }
    let mut result = vec![iterations / parts as u64; parts as usize - 1];
    let sum: u64 = result.iter().sum();
    result.push(iterations - sum);
    result
}
