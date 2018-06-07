use example::green_palette;
use render::filter::FilterType;
use template::builders::transform;
use template::filter_builder;
use template::FilterConfig;
use template::flame::CameraConfig;
use template::flame::Flame;
use template::flame::RenderConfig;
use template::palette::Palette;
use transforms::Transform;
use transforms::TransformSystem;
use util::math::RealPoint;
use variations::Variation;
use variations::Variations;
use variations::VariationType::Linear;

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


fn variations() -> Variations {
    Variations::new(vec![Variation { variation_type: Linear, weight: 1.0 }])
}


fn transforms() -> Vec<Transform> {
    vec![
        transform(1.0, 0.7, B1, variations()),
        transform(24.0, 0.8, B2, variations()),
        transform(3.0, 0.9, B3, variations()),
        transform(3.0, 0.9, B4, variations()),
    ]
}

pub fn get_flame_template() -> Flame {
    let mut render: RenderConfig = RenderConfig {
        width: 1920,
        height: 1080,
        quality: 400,
        oversampling: 3,
        brightness: 4.0,
        border: 0,
    };
    let camera: CameraConfig = CameraConfig {
        origin: RealPoint(-6.0, -0.5),
        scale_x: 12.0,
        scale_y: 12.0,
    };


    let filter_config: FilterConfig = FilterConfig {
        filter_type: FilterType::Gaussian,
        radius: 0.75,
    };
    let filter = filter_builder::filter(&filter_config, render.oversampling);
    render.border = (filter.width - render.oversampling).max(0);

    let transforms = TransformSystem::new(transforms());
    let palette: Palette = green_palette::palette();

    Flame { render, camera, filter, transforms, palette }
}