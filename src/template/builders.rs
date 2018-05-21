use camera::Camera;
use canvas::Canvas;
use template::flame_template::CameraConfig;
use template::flame_template::RenderConfig;
use template::flame_template::TransformTemplate;
use transforms::TransformSystem;
use util::math::TransformMatrix;

pub fn camera(config: &CameraConfig) -> Camera {
    Camera::new(config.origin, config.scale)
}

pub fn canvas(config: &RenderConfig) -> Canvas {
    Canvas::new(config.width, config.height, config.quality)
}

pub fn iterations(config: &RenderConfig) -> u32 {
    config.width * config.height * config.quality
}

pub fn transform(weight: f64, color: f64, affine_coefficients: [f64; 6]) -> TransformTemplate {
    TransformTemplate{weight, color, affine_coefficients}
}

pub fn transform_system(templates: &Vec<TransformTemplate>) -> TransformSystem {
    let mut transforms = vec![];
    let total_weight: f64 = templates.iter()
        .map(|t| t.weight)
        .sum();
    assert_ne!(0.0, total_weight, "Incorrect set of transforms: weight is zero!");
    for template in templates {
        let cf = template.affine_coefficients;
        let affine: TransformMatrix = TransformMatrix(
            (cf[0], cf[1], cf[2]),
            (cf[3], cf[4], cf[5]),
            (0.0, 0.0, 1.0),
        );
        transforms.push(( affine,  template.weight, template.color,));
    }
    TransformSystem::new(transforms)
}