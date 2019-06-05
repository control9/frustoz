use crate::util::math::RealPoint;
use crate::model::palette::Palette;
use crate::render::filter::FilterKernel;
use crate::transforms::TransformSystem;

#[derive(Clone, Debug)]
pub struct Flame {
    pub render: RenderConfig,
    pub camera: CameraConfig,
    pub filter: FilterKernel,
    pub transforms: TransformSystem,
    pub palette: Palette,
}

#[derive(Copy, Clone, Debug)]
pub struct CameraConfig {
    pub origin: RealPoint,
    pub scale_x: f64,
    pub scale_y: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct RenderConfig {
    pub width: u32,
    pub height: u32,
    pub quality: u32,
    pub oversampling: u32,
    pub brightness: f64,
    pub border: u32,
}
