use render::render_task::RenderTask;
use template::flame_template::FlameTemplate;
use render::histogram_processor::HistogramProcessor;
use render::canvas::HistogramLayer;
use render::spatial_filter;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

use std::time::Instant;

pub struct Renderer {
    pub threads: u32,
}

impl Renderer {
    pub fn render(&self, flame: &FlameTemplate) -> Vec<u8> {

        let now = Instant::now();
        ThreadPoolBuilder::new().num_threads(self.threads as usize).build_global().expect("Failed to initialize pool");
        let tasks : Vec<RenderTask> = (0..self.threads)
            .into_par_iter()
            .map(|_| RenderTask::new(&flame, self.threads) )
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