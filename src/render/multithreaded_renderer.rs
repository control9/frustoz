use render::render_task::RenderTask;
use template::flame_template::FlameTemplate;
use render::histogram_processor::HistogramProcessor;
use render::canvas::Histogram;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

pub struct Renderer {
    pub threads: u32,
}

impl Renderer {
    pub fn render(&self, flame: &FlameTemplate) -> Vec<u8> {
        ThreadPoolBuilder::new().num_threads(self.threads as usize).build_global().expect("Failed to initialize pool");
        let tasks : Vec<RenderTask> = (0..self.threads)
            .map(|_| RenderTask::new(&flame, self.threads) )
            .collect();

        let histograms : Vec<Histogram> = tasks.into_par_iter()
            .map(|t| t.render())
            .collect();

        let render = &flame.render;
        let processor = HistogramProcessor::new(render.quality, render.width, render.height, render.oversampling);
        processor.process_to_raw(histograms)
    }
}