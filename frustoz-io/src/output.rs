use image::ImageBuffer;

use image;

pub fn write(filename: &str, data: Vec<u8>, image_width: u32, image_height: u32) {

    let res: Option<ImageBuffer<image::Rgb<u8>, Vec<u8>>> = ImageBuffer::from_vec(image_width, image_height, data);
    match res {
        Some(im) => image::ImageRgb8(im).save(filename).expect("Failed to write file"),
        None => panic!("Unexpected error")
    };
}