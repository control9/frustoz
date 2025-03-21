use frustoz_core::model::flame::Flame;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::str::FromStr;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;
use xml::EventReader;

mod camera;
mod flame;
mod palette;
mod render;
mod transform;

pub fn parse_file(path: &str) -> Vec<Flame> {
    let file = File::open(path).unwrap();
    let mut file = BufReader::new(file);
    let mut r = Vec::new();
    file.read(&mut r).expect("TODO: panic message");

    let mut reader = EventReader::new(&r[..]);
    let mut result = vec![];

    loop {
        match reader.next() {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.local_name.eq_ignore_ascii_case("flame") {
                    let flame = flame::parse_flame(&mut reader, extract_attributes(attributes));
                    trace!("{:?}", flame);
                    result.push(flame);
                }
            }
            Ok(XmlEvent::EndDocument) => break,
            Err(_) => break,
            _ => continue,
        }
    }

    result
}

fn extract<T>(name: &str, default: T, attributes: &HashMap<String, String>) -> T
where
    T: FromStr,
{
    attributes
        .get(name)
        .map_or(default, |x| x.parse::<T>().ok().unwrap())
}

fn extract_all<T>(name: &str, default: &str, attributes: &HashMap<String, String>) -> Vec<T>
where
    T: FromStr,
{
    attributes
        .get(name)
        .map_or(default, String::as_str)
        .split(' ')
        .map(|x| x.parse::<T>().ok().unwrap())
        .collect()
}

fn extract_attributes(attributes_vec: Vec<OwnedAttribute>) -> HashMap<String, String> {
    attributes_vec
        .into_iter()
        .map(|a| (a.name.local_name, a.value))
        .collect()
}
