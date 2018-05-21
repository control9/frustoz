use template::flame_template::RenderConfig;
use image::ImageBuffer;

use image;

pub fn write(filename: &str, data: Vec<u8>, render: &RenderConfig) {

    let res: Option<ImageBuffer<image::Rgb<u8>, Vec<(u8)>>> = ImageBuffer::from_vec(render.width, render.height, data);
    match res {
        Some(im) => image::ImageRgb8(im).save(filename).expect("Failed to write file"),
        None => panic!("Unexpected error")
    };
}