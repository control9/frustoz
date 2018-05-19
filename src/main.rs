extern crate image;
extern crate rand;

use camera::Camera;
use canvas::Canvas;
use math::RealPoint;
use math::TransformMatrix;
use math::ProjectivePoint;
use image::ImageBuffer;
use image::Luma;
use rand::Rng;
use rand::ThreadRng;
use std::path::Path;

pub mod camera;
pub mod canvas;
pub mod math;
pub mod coordinates;
pub mod transforms;

const WARMUP_ITERATIONS: u32 = 20;
const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

const QUALITY: u32 = 800;
const ITERATIONS: u32 = QUALITY * WIDTH * HEIGHT;

fn main() {
    let mut rng = rand::thread_rng();

    let xstart: f64 = rng.gen_range(0.0, 1.0);
    let ystart: f64 = rng.gen_range(0.0, 1.0);

    let mut point = RealPoint(xstart, ystart);
    let camera = Camera::new(RealPoint(-1.0, -1.0), 2.5, 2.5);
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let variations = &transforms::SIERPINSKI_CARPET;

    for iteration in 1..ITERATIONS {
        point = apply(&point, &mut rng, variations);
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

fn apply(point: &RealPoint, rng: &mut ThreadRng, transforms: &[TransformMatrix]) -> RealPoint {
    let count = transforms.len();
    let var = rng.gen_range(0, count);

    let pr : ProjectivePoint = point.into();

    let transform : &TransformMatrix = transforms.get(var).expect("Generated incorrect transform ID");
    let result_pr = &(transform * &pr);
    let result : RealPoint = result_pr.into();
    result
}


