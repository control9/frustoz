use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use render::filter::FilterKernel;
use render::histogram::canvas::HistogramLayer;
use render::histogram_processor::HistogramProcessor;
use render::progress_bar;
use render::render_task::RenderTask;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::time::Instant;
use template::builders;
use template::filter_builder;
use template::flame_template::FlameTemplate;
use template::flame_template::RenderConfig;

pub struct Renderer {
    pub threads: u32,
}

impl Renderer {
    pub fn render(&self, flame: &mut FlameTemplate) -> Vec<u8> {
        let now = Instant::now();
        ThreadPoolBuilder::new().num_threads(self.threads as usize).build_global().expect("Failed to initialize pool");

        let filter = filter_builder::filter(&flame.filter, flame.render.oversampling);
        {
            let render = &mut flame.render;
            render.border = (filter.width - render.oversampling).max(0);
        }
        let processor = create_histogram_processor(&flame.render, flame.camera.scale_x, flame.camera.scale_y, filter);

        let iterations = builders::iterations(&flame.render);
        let iterations_per_thread = split(iterations, self.threads);

        let (tx, rx) = mpsc::channel();
        progress_bar::console_progress_bar(rx, iterations, 5);

        let thread_configs: Vec<(u32, Sender<u32>)> = iterations_per_thread.iter()
            .map(|&i| (i, tx.clone()))
            .collect();

        let tasks: Vec<RenderTask> = thread_configs.into_par_iter()
            .map(move |(iters, tx)| RenderTask::new(&flame, iters, tx))
            .collect();

        let elapsed = now.elapsed();
        println!("Creating tasks took: {:?}", (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0));


        let histograms: Vec<HistogramLayer> = tasks.into_par_iter()
            .map(|t| t.render())
            .collect();


        processor.process_to_raw(histograms)
    }
}

fn create_histogram_processor(config: &RenderConfig, view_width: f64, view_height: f64, filter: FilterKernel) -> HistogramProcessor {
    let histogram_width = config.width * config.oversampling + config.border;
    let histogram_height = config.height * config.oversampling + config.border;

    HistogramProcessor::new(
        config.quality,
        config.width, config.height,
        histogram_width, histogram_height,
        view_width, view_height,
        config.oversampling, config.brightness,
        filter,
    )
}

fn split(iterations: u32, parts: u32) -> Vec<u32> {
    if parts == 1 {
        return vec![iterations];
    }
    let mut result = vec![iterations / parts; parts as usize - 1];
    let sum: u32 = result.iter().sum();
    result.push(iterations - sum);
    result
}
