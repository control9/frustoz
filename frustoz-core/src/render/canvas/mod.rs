use crate::util::math::RealPoint;

pub mod camera;
mod canvas;

pub struct Camera {
    origin: RealPoint,
    scale_x: f64,
    scale_y: f64,
}
