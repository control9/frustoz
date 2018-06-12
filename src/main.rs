extern crate image;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate rand;
extern crate rayon;
extern crate simplelog;
extern crate xml;
extern crate pbr;

use rayon::ThreadPoolBuilder;
use simplelog::*;
use std::env;
use std::fs::File;
use std::time::Instant;

pub mod render;
pub mod example;
pub mod template;
pub mod transforms;
pub mod output;
pub mod util;
pub mod variations;
mod parser;

const PRESERVE_CPUS: u32 = 1;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default()).unwrap(),
            WriteLogger::new(LevelFilter::Debug, Config::default(), File::create("frustoz.log").unwrap()),
        ]
    ).unwrap();
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
    info!("Time elapsed: {:?}", (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0));
}


