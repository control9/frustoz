use rayon::prelude::*;
use super::Histogram;


// Easier switching between implementations for performance comparison
#[allow(unused_imports)]
use super::render_task::RenderTask;
#[allow(unused_imports)]
use super::split_render_task::SplitRenderTask;


use std::time::Instant;
use crate::model::builders;
use crate::model::flame::Flame;
use super::ProgressReporter;
pub struct Renderer {
    pub threads: u32,
}

type Task<T> = SplitRenderTask<T>;

impl Renderer {
    pub fn render<T: ProgressReporter + Clone + Send>(&self, flame: Flame) -> Vec<u8> {
        let now = Instant::now();

        let processor = builders::histogram_processor(&flame);

        let iterations = builders::iterations(&flame.render);
        let iterations_per_thread = split(iterations, self.threads);

        let reporter = T::new(&iterations_per_thread);
        let thread_configs: Vec<(u64, T, Flame)> = iterations_per_thread.iter()
            .map(|&i| (i, reporter.clone(), flame.clone()))
            .collect();

//        progress_bar::multi_progress_bar(rx, iterations, &iterations_per_thread);

        let tasks: Vec<Task<T>> = thread_configs.into_par_iter()
            .enumerate()
            .map(move |(i, (iters, rep, flame))| Task::new(flame, iters, i, rep))
            .collect();

        let elapsed = now.elapsed();
        info!("Creating tasks took: {:?}", (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0));


        let histograms: Vec<Histogram> = tasks.into_par_iter()
            .map(|t| t.render())
            .collect();


        processor.process_to_raw(histograms)
    }
}

fn split(iterations: u64, parts: u32) -> Vec<u64> {
    if parts == 1 {
        return vec![iterations];
    }
    let mut result = vec![iterations / parts as u64; parts as usize - 1];
    let sum: u64 = result.iter().sum();
    result.push(iterations - sum);
    result
}
