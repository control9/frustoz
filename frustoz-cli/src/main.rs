extern crate frustoz_core;
extern crate frustoz_io;
extern crate indicatif;
extern crate indicatif_log_bridge;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate pbr;
extern crate rayon;
extern crate simplelog;
extern crate tokio;

use frustoz_core::example;
pub use frustoz_core::render;
use frustoz_io::output;
use frustoz_io::parser;

use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use progress_bar::MultiProgressBar;
use rayon::ThreadPoolBuilder;
use simplelog::*;
use std::env;
use std::fs::File;
use std::sync::OnceLock;
use std::time::Instant;

mod progress_bar;

const PRESERVE_CPUS: u32 = 1;
static MB: OnceLock<MultiProgress> = OnceLock::new();

#[tokio::main]
async fn main() {
    let logger = CombinedLogger::new(vec![
        TermLogger::new(
            LevelFilter::Trace,
            Config::default(),
            TerminalMode::Stdout,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("frustoz.log").unwrap(),
        ),
    ]);

    LogWrapper::new(MB.get_or_init(|| MultiProgress::new()).clone(), logger)
        .try_init()
        .unwrap();
    let now = Instant::now();
    let threads = (num_cpus::get() as u32 - PRESERVE_CPUS).max(1);
    ThreadPoolBuilder::new()
        .num_threads(threads as usize)
        .build_global()
        .expect("Failed to initialize pool");

    let args: Vec<String> = env::args().collect();
    let models = match args.len() {
        1 => vec![example::spark()],
        _ => parser::parse_file(&args[1]),
    };

    for (num, model) in models.into_iter().enumerate() {
        let (image_width, image_height) = (model.render.width, model.render.height);

        #[cfg(feature = "async-rendering")]
        let raw = {
            info!("Running with async-rendering");
            let renderer = render::tokio_multithreaded_renderer::Renderer { threads };
            renderer.render::<MultiProgressBar>(model).await
        };

        #[cfg(not(feature = "async-rendering"))]
        let raw = {
            info!("Running without async-rendering");
            let renderer = render::multithreaded_renderer::Renderer { threads };
            renderer.render::<MultiProgressBar>(model)
        };

        output::write(
            &format!("fractal_{}.png", num + 1),
            raw,
            image_width,
            image_height,
        );
    }
    let elapsed = now.elapsed();
    info!(
        "Time elapsed: {:?}",
        (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0)
    );
}
