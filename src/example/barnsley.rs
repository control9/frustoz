use template::flame_template::TransformTemplate;
use builders::transform;
use template::flame_template::FlameTemplate;
use template::flame_template::RenderConfig;
use template::flame_template::CameraConfig;
use math::RealPoint;
use example::green_palette;
use template::palette::Palette;

const B1: [f64; 6] = [
    0.0, 0.0, 0.0,
    0.0, 0.16, 0.0,
];

const B2: [f64; 6] = [
    0.85, 0.04, 0.0,
    -0.04, 0.85, 1.6
];

const B3: [f64; 6] = [
    0.2, -0.26, 0.0,
    0.23, 0.22, 1.6
];

const B4: [f64; 6] = [
    -0.15, 0.28, 0.0,
    0.26, 0.24, 0.44
];


fn get_transform_templates() -> Vec<TransformTemplate> {
    vec![
        transform(1.0, 0.7, B1),
        transform(85.0, 0.5, B2),
        transform(7.0, 0.6, B3),
        transform(7.0, 0.6, B4),
    ]
}

pub fn get_flame_template() -> FlameTemplate {
    let render: RenderConfig = RenderConfig {
        width: 600,
        height: 800,
        quality: 100,
        skip_iterations: 20,
    };
    let camera: CameraConfig = CameraConfig {
        origin: RealPoint(-6.0, -0.5),
        scale: 12.0,
    };
    let transforms = get_transform_templates();
    let palette: Palette = green_palette::palette();

    FlameTemplate { render, camera, transforms, palette }
}