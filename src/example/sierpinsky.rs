use example::green_palette;
use render::filter::FilterType;
use template::builders::transform;
use template::flame::CameraConfig;
use template::flame::Flame;
use template::flame::RenderConfig;
use template::palette::Palette;
use util::math::RealPoint;
use template::TransformTemplate;
use template::FilterConfig;
use template::builders;
use template::filter_builder;

const S1: [f64; 6] = [
    0.5, 0.0, 0.0,
    0.0, 0.5, 0.0
];

const S2: [f64; 6] = [
    0.5, 0.0, 0.5,
    0.0, 0.5, 0.0
];

const S3: [f64; 6] = [
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

pub fn get_flame_template() -> Flame {
    let mut render: RenderConfig = RenderConfig {
        width: 1920,
        height: 1080,
        quality: 800,
        oversampling: 1,
        brightness: 4.0,
        skip_iterations: 20,
        border: 0,
    };
    let camera: CameraConfig = CameraConfig {
        origin: RealPoint(-0.05, -0.05),
        scale_x: 1.1,
        scale_y: 1.1,
    };

    let filter_config: FilterConfig = FilterConfig {
        filter_type: FilterType::Gaussian,
        radius: 0.75,
    };
    let filter = filter_builder::filter(&filter_config, render.oversampling);
    render.border = (filter.width - render.oversampling).max(0);
    let transforms = builders::transform_system(&get_transform_templates());
    let palette: Palette = green_palette::palette();

    Flame { render, camera, filter, transforms, palette }
}
