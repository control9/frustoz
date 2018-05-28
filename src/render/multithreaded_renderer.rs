use std::sync::mpsc;
use render::render_task::RenderTask;
use template::flame_template::FlameTemplate;
use render::histogram_processor::HistogramProcessor;
use render::canvas::HistogramLayer;
use render::spatial_filter;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

use std::time::Instant;
use template::builders;
use render::progress_bar;
use std::sync::mpsc::Sender;

pub struct Renderer {
    pub threads: u32,
}

impl Renderer {
    pub fn render(&self, flame: &FlameTemplate) -> Vec<u8> {

        let now = Instant::now();
        ThreadPoolBuilder::new().num_threads(self.threads as usize).build_global().expect("Failed to initialize pool");

        let iterations = builders::iterations(&flame.render);
        let iterations_per_thread = split(iterations, self.threads);

        let (tx, rx) = mpsc::channel();
        progress_bar::console_progress_bar(rx, iterations, 5);

        let thread_configs : Vec<(u32, Sender<u32>)> = iterations_per_thread.iter()
            .map(|&i| (i, tx.clone()))
            .collect();

        let tasks : Vec<RenderTask> = thread_configs.into_par_iter()
            .map(move |(iters, tx)| RenderTask::new(&flame, iters, tx) )
            .collect();

        let elapsed = now.elapsed();
        println!("Creating tasks took: {:?}", (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0));


        let histograms : Vec<HistogramLayer> = tasks.into_par_iter()
            .map(|t| t.render())
            .collect();

        let render = &flame.render;
        let filter = spatial_filter::create_filter(0, render.oversampling, 0.75);
        let processor = HistogramProcessor::new(render.quality, render.width, render.height, render.oversampling, filter);
        processor.process_to_raw(histograms)
    }

}

fn split(iterations: u32, parts: u32) -> Vec<u32> {
    if parts == 1 {
        return vec![iterations];
    }
    let mut result = vec![iterations / parts; parts as usize - 1];
    let sum : u32 = result.iter().sum();
    result.push(iterations - sum);
    result
}
