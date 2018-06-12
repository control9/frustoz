use rayon::prelude::*;
use render::histogram::canvas::HistogramLayer;
use render::progress_bar;
use render::render_task::RenderTask;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::time::Instant;
use template::builders;
use template::flame::Flame;
use super::Progress;

pub struct Renderer {
    pub threads: u32,
}

impl Renderer {
    pub fn render(&self, flame: Flame) -> Vec<u8> {
        let now = Instant::now();

        let processor = builders::histogram_processor(&flame);

        let iterations = builders::iterations(&flame.render);
        let iterations_per_thread = split(iterations, self.threads);

        let (tx, rx) = mpsc::channel();
        let thread_configs: Vec<(u32, Sender<Progress>, Flame)> = iterations_per_thread.iter()
            .map(|&i| (i, tx.clone(), flame.clone()))
            .collect();

        progress_bar::multi_progress_bar(rx, iterations, &iterations_per_thread);

        let tasks: Vec<RenderTask> = thread_configs.into_par_iter()
            .enumerate()
            .map(move |(i, (iters, tx, flame))| RenderTask::new(flame, iters, i,tx))
            .collect();

        let elapsed = now.elapsed();
        info!("Creating tasks took: {:?}", (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0));


        let histograms: Vec<HistogramLayer> = tasks.into_par_iter()
            .map(|t| t.render())
            .collect();


        processor.process_to_raw(histograms)
    }
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
