use render::histogram_processor::HistogramProcessor;
use render::progress_bar;
use render::render_task::RenderTask;
use std::sync::mpsc;
use template::builders;
use template::filter_builder;
use template::flame_template::FlameTemplate;

pub fn render(flame: &mut FlameTemplate) -> Vec<u8> {
    let iterations = builders::iterations(&flame.render);
    let (tx, rx) = mpsc::channel();
    progress_bar::console_progress_bar(rx, iterations, 1);
    let filter = filter_builder::filter(&flame.filter, flame.render.oversampling);

    {
        let render = &mut flame.render;
        render.border = (filter.width - render.oversampling).max(0);
    }
    let render = &flame.render;

    let task: RenderTask = RenderTask::new(flame, iterations, tx);

    let histogram_width = render.width * render.oversampling + render.border;
    let histogram_height = render.height * render.oversampling + render.border;

    let processor = HistogramProcessor::new(
        render.quality, render.width, render.height, histogram_width, histogram_height, render.oversampling, filter
    );

    let histogram = task.render();
    processor.process_to_raw(vec![histogram])
}