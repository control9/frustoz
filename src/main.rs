extern crate image;
extern crate rand;

use camera::Camera;
use canvas::Canvas;
use coordinates::PlanePoint;
use image::ImageBuffer;
use image::Luma;
use rand::Rng;
use rand::ThreadRng;
use std::path::Path;

pub mod camera;
pub mod canvas;
pub mod math;
pub mod coordinates;

const WARMUP_ITERATIONS: u32 = 20;
const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

const QUALITY: u32 = 800;
const ITERATIONS: u32 = QUALITY * WIDTH * HEIGHT;

fn main() {
    let mut rng = rand::thread_rng();

    let xstart: f64 = rng.gen_range(0.0, 1.0);
    let ystart: f64 = rng.gen_range(0.0, 1.0);

    let mut point = PlanePoint(xstart, ystart);
    let camera = Camera::new(PlanePoint(-1.0, -1.0), 2.0, 2.0);
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    for iteration in 1..ITERATIONS {
        point = apply(point, &mut rng);
        if iteration > WARMUP_ITERATIONS {
            let camera_coordinates = camera.project(&point);
            canvas.project_and_update(&camera_coordinates);
        }
    }

    // Save the image as “fractal.png”
    let path = Path::new("fractal.png");
    let mapper = |v: &u64| ((*v as f64).log2() * 8.0) as u8;
    let vec = canvas.extract_pixels(mapper);

    let res: Option<ImageBuffer<Luma<u8>, Vec<u8>>> = ImageBuffer::from_raw(WIDTH, HEIGHT, vec);
    match res {
        Some(im) => image::ImageLuma8(im).save(path).expect("Failed to write file"),
        None => panic!("Unexpected error")
    };
}

fn apply(PlanePoint(x, y): PlanePoint, rng: &mut ThreadRng) -> PlanePoint {
    let var = rng.gen_range(0, 3);
    match var {
        0 => PlanePoint(x / 2.0, y / 2.0),
        1 => PlanePoint((x + 1.0) / 2.0, y / 2.0),
        2 => PlanePoint(x / 2.0, (y + 1.0) / 2.0),
        _ => PlanePoint(x, y)
    }
}


