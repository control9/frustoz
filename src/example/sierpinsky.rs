use template::flame_template::TransformTemplate;
use template::builders::transform;
use template::flame_template::FlameTemplate;
use template::flame_template::RenderConfig;
use template::flame_template::CameraConfig;
use util::math::RealPoint;
use example::green_palette;
use template::palette::Palette;

const S1: [f64;6] =[
    0.5, 0.0, 0.0,
    0.0, 0.5, 0.0
];

const S2: [f64;6] = [
    0.5, 0.0, 0.5,
    0.0, 0.5, 0.0
];

const S3: [f64;6] = [
    0.5, 0.0, 0.0,
    0.0, 0.5, 0.5
];


fn get_transform_templates() -> Vec<TransformTemplate> {
    vec![
        transform(1.0, 0.5, S1),
        transform(1.0, 0.5, S2),
        transform(1.0, 0.5, S3),
    ]
}

pub fn get_flame_template() -> FlameTemplate {
    let render: RenderConfig = RenderConfig {
        width: 1920,
        height: 1080,
        quality: 800,
        oversampling: 1,
        skip_iterations: 20,
    };
    let camera: CameraConfig = CameraConfig {
        origin: RealPoint(-0.05, -0.05),
        scale_x: 1.1,
        scale_y: 1.1,
    };
    let transforms = get_transform_templates();
    let palette: Palette = green_palette::palette();

    FlameTemplate { render, camera, transforms, palette }
}
