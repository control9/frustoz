use super::extract;
use frustoz_core::model::builders;
use frustoz_core::model::palette::Palette;
use std::collections::HashMap;

pub fn extract_palette(attributes: &HashMap<String, String>, palette_body: String) -> Palette {
    let palette = palette_body
        .lines()
        .map(str::trim)
        .collect::<Vec<&str>>()
        .concat();
    let size = extract("count", palette.len() as u32 / 6, attributes);
    builders::palette(size, &palette)
}
