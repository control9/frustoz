use crate::render::filter::FilterType;

pub mod builders;
pub mod filter_builder;
pub mod flame;
pub mod palette;

pub struct FilterConfig {
    pub filter_type: FilterType,
    pub radius: f64,
}

pub struct TransformTemplate {
    pub weight: f64,
    pub color: f64,
    pub affine_coefficients: [f64; 6],
}
