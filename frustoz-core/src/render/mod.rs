pub mod filter;
pub mod histogram;
pub mod histogram_processor;
pub mod multithreaded_renderer;
mod progressive_render_task;
pub mod progressive_renderer;
mod render_task;
pub mod simple_renderer;
pub mod split_render_task;
pub mod tokio_multithreaded_renderer;

#[derive(Clone)]
pub struct Histogram {
    data: Vec<HDRPixel>,
    width: u32,
    height: u32,
}

#[derive(Copy, Clone)]
pub struct HDRPixel(pub f64, pub f64, pub f64, pub f64);

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
