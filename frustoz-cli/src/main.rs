extern crate image;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate rand;
extern crate rayon;
extern crate simplelog;
extern crate xml;
extern crate pbr;

extern crate frustoz_core;
pub use frustoz_core::render;
pub use frustoz_core::model;
pub use frustoz_core::util;
pub use frustoz_core::variations;
pub use frustoz_core::transforms;
use frustoz_core::example;

use rayon::ThreadPoolBuilder;
use simplelog::*;
use std::env;
use std::fs::File;
use std::time::Instant;
use progress_bar::MultiProgressBar;

mod output;
mod parser;
mod progress_bar;

const PRESERVE_CPUS: u32 = 1;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Error, Config::default()).unwrap(),
            WriteLogger::new(LevelFilter::Debug, Config::default(), File::create("frustoz.log").unwrap()),
        ]
    ).unwrap();
    let now = Instant::now();
    let threads = (num_cpus::get() as u32 - PRESERVE_CPUS).max(1);
    ThreadPoolBuilder::new().num_threads(threads as usize).build_global().expect("Failed to initialize pool");

    let renderer = render::multithreaded_renderer::Renderer { threads };

    let args: Vec<String> = env::args().collect();
    let models = match args.len() {
        1 => vec![example::spark()],
        _ => parser::parse_file(&args[1]),
    };

    for (num, model) in models.into_iter().enumerate() {
        let (image_width, image_height) = (model.render.width, model.render.height);

        let raw = renderer.render::<MultiProgressBar>(model);
        output::write(&format!("fractal_{}.png", num + 1), raw, image_width, image_height);
    }
    let elapsed = now.elapsed();
    info!("Time elapsed: {:?}", (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0));
}


