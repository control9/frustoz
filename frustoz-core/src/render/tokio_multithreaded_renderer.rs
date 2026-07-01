use super::{split};
use std::sync::Arc;

use super::split_render_task::SplitRenderTask;

use super::ProgressReporter;
use crate::model::builders;
use crate::model::flame::Flame;
use futures::future::join_all;

use tokio::task::spawn_blocking;
use tokio::task::JoinHandle;
use tokio_with_wasm::alias as tokio;
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

        let processor = builders::canvas_processor(&flame);

        let iterations = builders::iterations(&flame.render);
        let iterations_per_thread = split(iterations, self.threads);

        let reporter = T::new(&iterations_per_thread);
        let thread_configs: Vec<(u64, T, Flame)> = iterations_per_thread
            .iter()
            .map(|&i| (i, reporter.clone(), flame.clone()))
            .collect();

        let canvas = Arc::new(builders::canvas(&flame.render, flame.filter.width));

        {
            let canvas = canvas.clone();
            let tasks: Vec<Task<T>> = thread_configs
                .into_iter()
                .enumerate()
                .map(move |(i, (iters, rep, flame))| {
                    Task::new(flame, iters, i, rep, canvas.clone())
                })
                .collect();

            let elapsed = now.elapsed();
            info!(
                "Creating tasks took: {:?}",
                (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0)
            );

            let handlers: Vec<JoinHandle<()>> = tasks
                .into_iter()
                .map(|t| spawn_blocking(move || t.render()))
                .collect();

            let _ = join_all(handlers).await;
        }
        processor.process_to_raw_single(canvas)
    }
}
