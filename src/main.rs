extern crate image;
extern crate rand;

use camera::Camera;
use canvas::Canvas;
use image::ImageBuffer;
use image::Luma;
use math::RealPoint;
use rand::Rng;
use std::path::Path;

pub mod camera;
pub mod canvas;
pub mod coordinates;
pub mod example;
pub mod math;
pub mod transforms;

const WARMUP_ITERATIONS: u32 = 20;
const WIDTH: u32 = 600;
const HEIGHT: u32 = 800;

const QUALITY: u32 = 200;
const ITERATIONS: u32 = QUALITY * WIDTH * HEIGHT;

fn main() {
    let mut rng = rand::thread_rng();

    let xstart: f64 = rng.gen_range(0.0, 1.0);
    let ystart: f64 = rng.gen_range(0.0, 1.0);

    let mut point = RealPoint(xstart, ystart);
    let camera = Camera::new(RealPoint(-6.0, -0.5), 12.0, 12.0);
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let transformations = example::ExampleTransformations::new();
    let variations = transformations.barnsley();

    for iteration in 1..ITERATIONS {
        let transform_seed: f64 = rng.gen_range(0.0, 1.0);
        point = variations.apply_transform(transform_seed, &point);
        if iteration > WARMUP_ITERATIONS {
            let camera_coordinates = camera.project(&point);
            canvas.project_and_update(&camera_coordinates);
        }
    }

    // Save the image as “fractal.png”
    let path = Path::new("fractal.png");
    let mapper = |&v: &u64| ((v as f64).log2() * 8.0) as u8;
    let vec = canvas.extract_pixels(mapper);

    let res: Option<ImageBuffer<Luma<u8>, Vec<u8>>> = ImageBuffer::from_raw(WIDTH, HEIGHT, vec);
    match res {
        Some(im) => image::ImageLuma8(im).save(path).expect("Failed to write file"),
        None => panic!("Unexpected error")
    };
}


