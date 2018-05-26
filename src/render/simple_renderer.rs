use render::render_task::RenderTask;
use template::flame_template::FlameTemplate;
use render::histogram_processor::HistogramProcessor;

pub fn render(flame: &FlameTemplate) -> Vec<u8>{
    let task : RenderTask = RenderTask::new(flame, 1);
    let render = &flame.render;
    let processor = HistogramProcessor::new(render.quality, render.width, render.height, render.oversampling);

    let histogram = task.render();
    processor.process_to_raw(vec![histogram])
}