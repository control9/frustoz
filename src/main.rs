extern crate image;
extern crate rand;

use camera::Camera;
use canvas::Canvas;
use image::ImageBuffer;
use math::RealPoint;
use rand::Rng;
use std::path::Path;

pub mod camera;
pub mod canvas;
pub mod coordinates;
pub mod example;
pub mod math;
pub mod color;
pub mod transforms;

const WARMUP_ITERATIONS: u32 = 20;
const WIDTH: u32 = 600;
const HEIGHT: u32 = 800;

const QUALITY: u32 = 800;
const ITERATIONS: u32 = QUALITY * WIDTH * HEIGHT;

fn main() {
    let mut rng = rand::thread_rng();

    let xstart: f64 = rng.gen_range(0.0, 1.0);
    let ystart: f64 = rng.gen_range(0.0, 1.0);

    let mut point = RealPoint(xstart, ystart);
    let mut color : f64 = rng.gen_range(0.0, 1.0);
    let camera = Camera::new(RealPoint(-6.0, -0.5), 12.0, 12.0);
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let transformations = example::ExampleTransformations::new();
    let variations = transformations.barnsley();
    let palette = example::green_palette::palette();

    for iteration in 1..ITERATIONS {
        let transform_seed: f64 = rng.gen_range(0.0, 1.0);
        let transform = variations.get_transformation(transform_seed);

        let (new_point, new_color) = transform.apply(&point, color);
        point = new_point;
        color = new_color;

        if iteration > WARMUP_ITERATIONS {
            let camera_coordinates = camera.project(&point);
            canvas.project_and_update(&camera_coordinates, palette.get_color(color));
        }
    }

    let path = Path::new("fractal.png");
    let raw_image = canvas.extract_raw();

    let res: Option<ImageBuffer<image::Rgb<u8>, Vec<(u8)>>> = ImageBuffer::from_vec(WIDTH, HEIGHT, raw_image);
    match res {
        Some(im) => image::ImageRgb8(im).save(path).expect("Failed to write file"),
        None => panic!("Unexpected error")
    };
}


