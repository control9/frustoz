extern crate image;
extern crate rand;

use rand::Rng;
use std::path::Path;

pub mod camera;

fn main() {
    let max_iterations = 256u16;

    let imgx = 40;
    let imgy = 40;

    let scalex = 4.0 / imgx as f32;
    let scaley = 4.0 / imgy as f32;

    let xgen : f64 = rand::thread_rng().gen();
    let ygen : f64 = rand::thread_rng().gen();


    // Create a new ImgBuf with width: imgx and height: imgy

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    {
        //let mut pixel = imgbuf.get_pixel_mut((xgen * imgx as f64) as u32, (ygen * imgy as f64) as u32);
        let pixel = image::Luma([42 as u8]);
        println!("{}", pixel.data[0]);
    }



    // Save the image as “fractal.png”
    let path = Path::new("fractal.png");


    // We must indicate the image's color type and what format to save as
    image::ImageLuma8(imgbuf).save(path);
}


