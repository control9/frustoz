use super::render_task::RenderTask;
use super::ProgressReporter;
use crate::model::builders;
use crate::model::flame::Flame;

pub fn render<T: ProgressReporter>(flame: Flame) -> Vec<u8> {
    let iterations = builders::iterations(&flame.render);
    let progress_reporter = T::new(&vec![iterations]);

    let processor = builders::histogram_processor(&flame);

    let task = RenderTask::new(flame.clone(), iterations, 0, progress_reporter);

    let histogram = task.render();
    processor.process_to_raw(vec![histogram])
}
