pub mod canvas;
pub mod camera;
pub mod render_task;
pub mod simple_renderer;
pub mod histogram_processor;
pub mod multithreaded_renderer;

pub mod spatial_filter;
mod gamma_filter;
mod log_filter;
mod log_scale_calculator;

pub type Histogram = Vec<HDRPixel>;

#[derive(Copy, Clone)]
pub struct RGBACounter(pub f64, pub f64, pub f64, pub u64);

#[derive(Copy, Clone)]
pub struct HDRPixel(pub f64, pub f64, pub f64, pub f64);


