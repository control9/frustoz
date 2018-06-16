extern crate hex;

use render::histogram::Camera;
use render::histogram_processor::HistogramProcessor;
use template::flame::CameraConfig;
use template::flame::Flame;
use template::flame::RenderConfig;
use template::palette::Palette;
use transforms::Transform;
use util::math::TransformMatrix;
use variations::Variations;
use render::Histogram;

pub fn camera(config: &CameraConfig) -> Camera {
    Camera::new(config.origin, config.scale_x, config.scale_y)
}

pub fn histogram(config: &RenderConfig, filter_width: u32) -> Histogram {
    Histogram::new(config.width, config.height, config.oversampling, filter_width)
}

pub fn iterations(config: &RenderConfig) -> u32 {
    config.width * config.height * config.quality
}

pub fn transform(weight: f64, color: f64, coef: [f64; 6], variations: Variations) -> Transform {
    let affine: TransformMatrix = TransformMatrix(
        (coef[0], coef[1], coef[2]),
        (coef[3], coef[4], coef[5]),
        (0.0, 0.0, 1.0),
    );
    Transform { weight, color, affine, variations }
}

pub fn histogram_processor(flame: &Flame) -> HistogramProcessor {
    let render = &flame.render;

    HistogramProcessor::new(
        render.quality,
        render.width, render.height,
        flame.camera.scale_x, flame.camera.scale_y,
        render.oversampling, render.brightness,
        &flame.filter,
    )
}


pub fn palette(size: u32, content: &str) -> Palette {
    let content = content.trim();

    let colors = hex::decode(content).expect("Incorrect palette");
    assert_eq!(3 * size, colors.len() as u32);
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
