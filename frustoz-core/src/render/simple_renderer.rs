use super::ProgressReporter;
use super::render_task::RenderTask;
use std::sync::mpsc;
use crate::template::builders;
use crate::template::flame::Flame;

pub fn render<T: ProgressReporter>(flame: Flame) -> Vec<u8> {
    let iterations = builders::iterations(&flame.render);
    let progress_reporter = T::new(&vec![iterations]);
//    let (tx, rx) = mpsc::channel();
//    progress_bar::single_progress_bar(rx, iterations);

    let processor = builders::histogram_processor(&flame);

    let task = RenderTask::new(flame.clone(), iterations, 0, progress_reporter);

    let histogram = task.render();
    processor.process_to_raw(vec![histogram])
}

