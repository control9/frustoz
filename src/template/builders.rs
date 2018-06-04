extern crate hex;

use render::histogram::Camera;
use render::histogram::canvas::Canvas;
use render::histogram_processor::HistogramProcessor;
use template::flame::CameraConfig;
use template::flame::Flame;
use template::flame::RenderConfig;
use template::palette::Palette;
use template::TransformTemplate;
use transforms::TransformSystem;
use util::math::TransformMatrix;

pub fn camera(config: &CameraConfig) -> Camera {
    Camera::new(config.origin, config.scale_x, config.scale_y)
}

pub fn canvas(config: &RenderConfig) -> Canvas {
    Canvas::new(config.width * config.oversampling + config.border, config.height * config.oversampling + config.border)
}

pub fn iterations(config: &RenderConfig) -> u32 {
    config.width * config.height * config.quality
}

pub fn transform(weight: f64, color: f64, affine_coefficients: [f64; 6]) -> TransformTemplate {
    TransformTemplate { weight, color, affine_coefficients }
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
        transforms.push((affine, template.weight, template.color, ));
    }
    TransformSystem::new(transforms)
}

pub fn histogram_processor(flame: &Flame) -> HistogramProcessor {
    let render = &flame.render;

    let histogram_width = render.width * render.oversampling + render.border;
    let histogram_height = render.height * render.oversampling + render.border;

    HistogramProcessor::new(
        render.quality,
        render.width, render.height,
        histogram_width, histogram_height,
        flame.camera.scale_x, flame.camera.scale_y,
        render.oversampling, render.brightness,
        &flame.filter,
    )
}


pub fn palette(size: i32, content: &str) -> Palette {
    let content = content.trim();

    let colors = hex::decode(content).expect("Incorrect palette");
    assert_eq!(3 * size, colors.len() as i32);
    Palette::new(&colors)
}


#[cfg(test)]
mod palette_builder_test {
    use template::palette::RGB;

    #[test]
    pub fn should_decode_palette() {
        let input = "B9EAEBC1EEEBC5F2EBC9F2EB";
        let size = 4;

        let result = super::palette(size, input);
        assert_eq!(&RGB(185.0 / 256.0, 234.0 / 256.0, 235.0 / 256.0), result.get_color(0.0))
    }
}
