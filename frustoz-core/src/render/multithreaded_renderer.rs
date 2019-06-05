use rayon::prelude::*;
//use super::progress_bar;
use super::Histogram;
use super::render_task::RenderTask;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::time::Instant;
use crate::template::builders;
use crate::template::flame::Flame;
use super::Progress;
use super::ProgressReporter;
use super::NoOpReporter;

pub struct Renderer {
    pub threads: u32,
}

impl Renderer {
    pub fn render<T: ProgressReporter + Clone + Send>(&self, flame: Flame) -> Vec<u8> {
        let now = Instant::now();

        let processor = builders::histogram_processor(&flame);

        let iterations = builders::iterations(&flame.render);
        let iterations_per_thread = split(iterations, self.threads);

        let reporter = T::new(&iterations_per_thread);
        let thread_configs: Vec<(u32, T, Flame)> = iterations_per_thread.iter()
            .map(|&i| (i, reporter.clone(), flame.clone()))
            .collect();

//        progress_bar::multi_progress_bar(rx, iterations, &iterations_per_thread);

        let tasks: Vec<RenderTask<T>> = thread_configs.into_par_iter()
            .enumerate()
            .map(move |(i, (iters, rep, flame))| RenderTask::new(flame, iters, i, rep))
            .collect();

        let elapsed = now.elapsed();
        info!("Creating tasks took: {:?}", (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0));


        let histograms: Vec<Histogram> = tasks.into_par_iter()
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
