pub mod filter;
pub mod histogram;
pub mod render_task;
pub mod simple_renderer;
pub mod histogram_processor;
pub mod multithreaded_renderer;

mod progress_bar;

pub const EPSILON: f64 = 0.0000001;

pub type Histogram = Vec<HDRPixel>;

#[derive(Copy, Clone)]
pub struct RGBACounter(pub f64, pub f64, pub f64, pub u64);

#[derive(Copy, Clone)]
pub struct HDRPixel(pub f64, pub f64, pub f64, pub f64);


