use render::render_task::RenderTask;
use template::flame_template::FlameTemplate;
use render::histogram_processor::HistogramProcessor;

pub fn render(flame: &FlameTemplate) -> Vec<u8>{
    let task : RenderTask = RenderTask::new(flame);
    let processor = HistogramProcessor::new(flame.render.quality);

    let histogram = task.render();
    processor.process_to_raw(vec![histogram])
}