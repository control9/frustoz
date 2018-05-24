use util::math::RealPoint;
use template::palette::Palette;

pub struct FlameTemplate {
    pub render: RenderConfig,
    pub camera: CameraConfig,
    pub transforms: Vec<TransformTemplate>,
    pub palette: Palette,
}

pub struct TransformTemplate {
    pub weight: f64,
    pub color: f64,
    pub affine_coefficients: [f64; 6],
}

pub struct CameraConfig {
    pub origin: RealPoint,
    pub scale_x: f64,
    pub scale_y: f64,
}

pub struct RenderConfig {
    pub width: u32,
    pub height: u32,
    pub quality: u32,
    pub oversampling: u32,
    pub skip_iterations: u32,
}