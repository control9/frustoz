pub mod gamma_filter;
pub mod log_filter;
pub mod filter_type;
pub mod spatial_filter;

pub mod log_scale_calculator;

pub struct FilterKernel {
    pub width: u32,
    pub coefficients: Vec<f64>,
}

pub struct LogFilter {
    pub scale_calculator: LogScaleCalculator,
    pub width: u32,
    pub height: u32,
    pub oversampling: u32,
    pub white_level: f64,
}

pub struct LogScaleCalculator {
    k1: f64,
    k2: f64,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum FilterType {
    Gaussian,
    Hermite,
}