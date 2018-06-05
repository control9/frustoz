use template::flame::Flame;
use std::fs::File;
use std::io::BufReader;
use std;
use xml::reader::{EventReader, XmlEvent};
use xml::reader::Events;
use xml::attribute::OwnedAttribute;
use std::collections::HashMap;
use template::flame::RenderConfig;
use template::flame::CameraConfig;
use util::math::RealPoint;
use template::FilterConfig;
use render::filter::FilterType;
use template::filter_builder;
use template::palette::Palette;
use transforms::TransformSystem;

pub fn parse(_path: &str) -> Vec<Flame> {
    let file = File::open("file.xml").unwrap();
    let file = BufReader::new(file);

    let mut events = EventReader::new(file).into_iter();
    let mut result = vec![];

    loop {
        match events.next() {
            None => break,
            Some(Ok(XmlEvent::StartElement {name, attributes, ..})) => {
                result.push(parse_flame(&mut events, attributes));
            },
            _ => continue,
        }
    }

    result
}

const DEFAULT_SIZE: &str = "1920 1080";

fn extract<T>(name: &str, default: T, attributes: &HashMap<String, String>) -> T
    where T: std::str::FromStr
{
    attributes.get(name).map_or(default, |x| x.parse::<T>().ok().unwrap())
}

fn extract_all<T>(name: &str, default: &String, attributes: &HashMap<String, String>) -> Vec<T>
    where T: std::str::FromStr
{
    attributes.get(name)
        .unwrap_or(default)
        .split(' ')
        .map(|x| x.parse::<T>().ok().unwrap())
        .collect()
}

fn parse_flame(events: &mut Events<BufReader<File>>, attributes_vec: Vec<OwnedAttribute>) -> Flame {
    let attributes : HashMap<String, String> = attributes_vec.into_iter()
        .map(|a| (a.name.local_name, a.value))
        .collect();

    let default_size = DEFAULT_SIZE.to_string();

    let dimensions = extract_all("size", &default_size, &attributes);
    let (width, height) = (dimensions[0], dimensions[1]);

    let oversampling = extract("oversample", 2, &attributes);
    let quality = extract("quality", 100, &attributes);
    let brightness = extract("brightness", 4.0, &attributes);

    let mut render = RenderConfig {
        width,
        height,
        quality,
        oversampling,
        brightness,
        border: 0,
    };

    let pixels_per_unit = extract("scale", 100.0, &attributes);


    let camera: CameraConfig = CameraConfig {
        origin: RealPoint(-7.1282, -3.0393),
        scale_x: 12.355,
        scale_y: 6.95,
    };
    let filter_config: FilterConfig = FilterConfig {
        filter_type: FilterType::Gaussian,
        radius: 0.75,
    };
    let filter = filter_builder::filter(&filter_config, render.oversampling);
    render.border = (filter.width - render.oversampling).max(0);


    Flame {
        render,
        camera,
        filter,
        transforms: TransformSystem::new(vec![]),
        palette: Palette::new("B9EAEBC1EEEBC5F2EBC9F2EB".as_bytes()),
    }
}
