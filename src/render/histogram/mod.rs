use util::math::RealPoint;

pub mod camera;
pub mod histogram;

pub struct Camera {
    origin: RealPoint,
    scale_x: f64,
    scale_y: f64,
}

