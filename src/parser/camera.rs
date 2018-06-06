use std::collections::HashMap;
use super::extract;
use super::extract_all;
use template::flame::CameraConfig;
use util::math::RealPoint;

pub fn extract_camera_config(attributes: &HashMap<String, String>, image_width: f64, image_height: f64) -> CameraConfig {
    let pixels_per_unit = extract("scale", 100.0, attributes);
    let (scale_x, scale_y) = (image_width / pixels_per_unit, image_height / pixels_per_unit);

    let center: Vec<f64> = extract_all("center", "0.0 0.0", attributes);
    let (center_x, center_y) = (center[0], center[1]);
    let origin = RealPoint(center_x - scale_x / 2.0, center_y - scale_y / 2.0);

    CameraConfig {
        origin,
        scale_x,
        scale_y,
    }
}