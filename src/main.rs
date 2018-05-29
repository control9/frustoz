extern crate image;
extern crate rand;
extern crate rayon;
extern crate num_cpus;

use template::flame_template::FlameTemplate;
use std::time::Instant;

pub mod render;
pub mod example;
pub mod template;
pub mod transforms;
pub mod output;
pub mod util;

const PRESERVE_CPUS : u32 = 1;

fn main() {
    let now = Instant::now();
    let threads = (num_cpus::get() as u32 - PRESERVE_CPUS).max(1);
    let mut template: FlameTemplate = example::spark();
    let renderer = render::multithreaded_renderer::Renderer { threads };
    let raw = renderer.render(&mut template);
//    let raw = render::simple_renderer::render(&template);
    output::write("fractal.png", raw, &template.render);
    let elapsed = now.elapsed();
    println!("Time elapsed: {:?}", (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0));
}


