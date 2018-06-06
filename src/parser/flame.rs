use parser::palette;
use parser::transform;
use render::filter::FilterType;
use std::collections::HashMap;
use std::io::Read;
use super::camera;
use super::render;
use template::builders;
use template::filter_builder;
use template::FilterConfig;
use template::flame::Flame;
use template::palette::Palette;
use template::TransformTemplate;
use transforms::TransformSystem;
use xml::EventReader;
use xml::reader::XmlEvent;

pub fn parse_flame<_R: Read>(reader: &mut EventReader<_R>, attributes: HashMap<String, String>) -> Flame {
    let mut render = render::extract_render_config(&attributes);
    let camera = camera::extract_camera_config(&attributes, render.width as f64, render.height as f64);

    let filter_config: FilterConfig = FilterConfig {
        filter_type: FilterType::Gaussian,
        radius: 0.75,
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
    let mut transforms: Vec<TransformTemplate> = vec![];
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
    (builders::transform_system(&transforms), palette.unwrap_or(builders::palette(2, "000000FFFFFF")))
}