use render::progress_bar;
use render::render_task::RenderTask;
use std::sync::mpsc;
use template::builders;
use template::flame::Flame;

pub fn render(flame: Flame) -> Vec<u8> {
    let iterations = builders::iterations(&flame.render);
    let (tx, rx) = mpsc::channel();
    progress_bar::single_progress_bar(rx, iterations);

    let processor = builders::histogram_processor(&flame);

    let task: RenderTask = RenderTask::new(flame.clone(), iterations, 0, tx);

    let histogram = task.render();
    processor.process_to_raw(vec![histogram])
}

