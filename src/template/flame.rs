use util::math::RealPoint;
use template::palette::Palette;
use render::filter::FilterKernel;
use transforms::TransformSystem;

#[derive(Clone)]
pub struct Flame {
    pub render: RenderConfig,
    pub camera: CameraConfig,
    pub filter: FilterKernel,
    pub transforms: TransformSystem,
    pub palette: Palette,
}

#[derive(Copy, Clone)]
pub struct CameraConfig {
    pub origin: RealPoint,
    pub scale_x: f64,
    pub scale_y: f64,
}

#[derive(Copy, Clone)]
pub struct RenderConfig {
    pub width: u32,
    pub height: u32,
    pub quality: u32,
    pub oversampling: u32,
    pub brightness: f64,
    pub border: u32,
}
