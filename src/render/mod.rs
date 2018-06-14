pub mod filter;
pub mod histogram;
pub mod render_task;
pub mod simple_renderer;
pub mod histogram_processor;
pub mod multithreaded_renderer;

mod progress_bar;

pub type Histogram = Vec<HDRPixel>;

#[derive(Copy, Clone)]
pub struct HDRPixel(pub f64, pub f64, pub f64, pub f64);

#[derive(Copy, Clone)]
pub struct Progress(pub u32, pub usize);


