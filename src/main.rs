extern crate image;
extern crate num_cpus;
extern crate rand;
extern crate rayon;
extern crate xml;

use rayon::ThreadPoolBuilder;
use std::env;
use std::time::Instant;

pub mod render;
pub mod example;
pub mod template;
pub mod transforms;
pub mod output;
pub mod util;
mod parser;

const PRESERVE_CPUS: u32 = 1;

fn main() {
    let now = Instant::now();
    let threads = (num_cpus::get() as u32 - PRESERVE_CPUS).max(1);
    ThreadPoolBuilder::new().num_threads(threads as usize).build_global().expect("Failed to initialize pool");

    let renderer = render::multithreaded_renderer::Renderer { threads };

    let args: Vec<String> = env::args().collect();
    let templates = match args.len() {
        1 => vec![example::spark()],
        _ => parser::parse_file(&args[1]),
    };

    for (num, template) in templates.into_iter().enumerate() {
        let (image_width, image_height) = (template.render.width, template.render.height);

        let raw = renderer.render(template);
        output::write(&format!("fractal_{}.png", num + 1), raw, image_width, image_height);
    }
    let elapsed = now.elapsed();
    println!("Time elapsed: {:?}", (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0));
}


