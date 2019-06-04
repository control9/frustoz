use super::extract;
use super::extract_all;
use std::collections::HashMap;
use template::flame::RenderConfig;

const DEFAULT_SIZE: &str = "1920 1080";

pub fn extract_render_config(attributes: &HashMap<String, String>) -> RenderConfig {
    let dimensions = extract_all("size", DEFAULT_SIZE, attributes);
    let (width, height) = (dimensions[0], dimensions[1]);

    let oversampling = extract("oversample", 2, attributes);
    let quality = extract("quality", 100.0, attributes) as u32;
    let brightness = extract("brightness", 4.0, attributes);

    RenderConfig {
        width,
        height,
        quality,
        oversampling,
        brightness,
        border: 0,
    }
}