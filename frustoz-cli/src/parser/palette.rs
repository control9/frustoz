use std::collections::HashMap;
use template::palette::Palette;
use super::extract;
use template::builders;

pub fn extract_palette(attributes: &HashMap<String, String>, palette_body: String) -> Palette {
    let palette = palette_body.lines().collect::<Vec<&str>>().concat();
    let size = extract("count", palette.len() as u32 /6, attributes);
    builders::palette(size, &palette)
}