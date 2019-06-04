use super::palette;
use super::transform;
use crate::render::filter::FilterType;
use std::collections::HashMap;
use std::io::Read;
use super::camera;
use super::extract;
use super::render;
use crate::template::builders;
use crate::template::filter_builder;
use crate::template::FilterConfig;
use crate::template::flame::Flame;
use crate::template::palette::Palette;
use crate::transforms::Transform;
use crate::transforms::TransformSystem;
use xml::EventReader;
use xml::reader::XmlEvent;

pub fn parse_flame<_R: Read>(reader: &mut EventReader<_R>, attributes: HashMap<String, String>) -> Flame {
    let mut render = render::extract_render_config(&attributes);
    let camera = camera::extract_camera_config(&attributes, render.width as f64, render.height as f64);

    let filter_radius = extract("filter", 0.75, &attributes);
    let filter_type_name = attributes.get("filter_kernel").map(|x| x.to_uppercase());

    let filter_type = match filter_type_name.as_ref().map(String::as_str) {
        Some("HERMITE") => FilterType::Hermite,
        Some("BOX") => FilterType::Box,
        Some("TRIANGLE") => FilterType::Triangle,
        Some("BELL") => FilterType::Bell,
        Some("B_SPLINE") => FilterType::BSpline,
        Some("MITCHELL") => FilterType::Mitchell,
        Some("MITCHELL_SINEPOW") => FilterType::Mitchell,
        Some("BLACKMAN") => FilterType::Blackman,
        Some("GAUSSIAN") => FilterType::Gaussian,
        _ => FilterType::Gaussian,
    };

    let filter_config: FilterConfig = FilterConfig {
        filter_type: filter_type,
        radius: filter_radius,
    };

    let filter = filter_builder::filter(&filter_config, render.oversampling);
    render.border = (filter.width - render.oversampling).max(0);

    let (transforms, palette) = parse_sub_elements(reader);

    Flame {
        render,
        camera,
        filter,
        transforms,
        palette,
    }
}

fn parse_sub_elements<_R: Read>(reader: &mut EventReader<_R>) -> (TransformSystem, Palette) {
    let mut transforms: Vec<Transform> = vec![];
    let mut palette = None;
    loop {
        match reader.next() {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match name.local_name.as_str() {
                    "xform" => {
                        transforms.push(transform::extract_transform(&super::extract_attributes(attributes)));
                    }
                    "palette" => {
                        if let Ok(XmlEvent::Characters(palette_body)) = reader.next() {
                            palette = Some(
                                palette::extract_palette(&super::extract_attributes(attributes), palette_body)
                            );
                        }
                    }
                    _ => continue,
                }
            }
            Ok(XmlEvent::EndElement { ref name }) if name.local_name.eq_ignore_ascii_case("flame") => break,
            Ok(XmlEvent::EndDocument) => break,
            Err(_) => break,
            _ => continue,
        }
    }
    (TransformSystem::new(transforms), palette.unwrap_or(builders::palette(2, "000000FFFFFF")))
}