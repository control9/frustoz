use std::fmt::{Debug, Error, Formatter};

pub mod filter_type;
pub mod gamma_filter;
pub mod log_filter;
pub mod spatial_filter;

#[derive(Clone)]
pub struct FilterKernel {
    pub width: u32,
    pub coefficients: Vec<f64>,
}

impl Debug for FilterKernel {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Filter width [{}]", self.width)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LogFilter {
    k1: f64,
    k2: f64,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum FilterType {
    Gaussian,
    Hermite,
    Box,
    Triangle,
    Bell,
    BSpline,
    Mitchell,
    Blackman,
}
