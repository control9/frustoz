use std::ops::Add;

pub mod canvas;
pub mod camera;
pub mod render_task;
pub mod simple_renderer;
pub mod histogram_processor;
pub mod multithreaded_renderer;

#[derive(Copy, Clone)]
pub struct HDRPixel(pub f64, pub f64, pub f64, pub f64);

impl Add<HDRPixel> for HDRPixel {
    type Output = HDRPixel;

    fn add(self, HDRPixel(r2,g2,b2,a2): HDRPixel) -> HDRPixel {
        let HDRPixel(r1,g1,b1,a1) = self;
        HDRPixel(r1 + r2, g1 + g2, b1 + b2, a1 + a2)
    }
}
