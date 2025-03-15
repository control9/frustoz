use super::{split, Histogram};

// Easier switching between implementations for performance comparison
#[allow(unused_imports)]
use super::render_task::RenderTask;
#[allow(unused_imports)]
use super::split_render_task::SplitRenderTask;

use super::ProgressReporter;
use crate::model::builders;
use crate::model::flame::Flame;
use futures::future::join_all;
use tokio::task::spawn_blocking;
use tokio_with_wasm::tokio;
use tokio_with_wasm::tokio::task::JoinHandle;
use web_time::Instant;
pub struct Renderer {
    pub threads: u32,
}

type Task<T> = SplitRenderTask<T>;

impl Renderer {
    pub async fn render<T: ProgressReporter + Clone + Send + 'static>(
        &self,
        flame: Flame,
    ) -> Vec<u8> {
        let now = Instant::now();

        let processor = builders::histogram_processor(&flame);

        let iterations = builders::iterations(&flame.render);
        let iterations_per_thread = split(iterations, self.threads);

        let reporter = T::new(&iterations_per_thread);
        let thread_configs: Vec<(u64, T, Flame)> = iterations_per_thread
            .iter()
            .map(|&i| (i, reporter.clone(), flame.clone()))
            .collect();

        let tasks: Vec<Task<T>> = thread_configs
            .into_iter()
            .enumerate()
            .map(move |(i, (iters, rep, flame))| Task::new(flame, iters, i, rep))
            .collect();

        let elapsed = now.elapsed();
        info!(
            "Creating tasks took: {:?}",
            (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0)
        );

        let handlers: Vec<JoinHandle<Histogram>> = tasks
            .into_iter()
            .map(|t| spawn_blocking(move || t.render()))
            .collect();

        let hists = join_all(handlers).await;
        let histograms: Vec<Histogram> = hists.into_iter().filter_map(|r| r.ok()).collect();
        processor.process_to_raw(histograms)
    }
}
