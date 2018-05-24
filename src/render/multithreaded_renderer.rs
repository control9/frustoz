use std::thread;
use std::sync::mpsc::channel;
use render::render_task::RenderTask;
use template::flame_template::FlameTemplate;
use render::histogram_processor::HistogramProcessor;
use render::canvas::Histogram;

pub struct Renderer {
    pub threads: u32,
}

impl Renderer {
    pub fn render(&self, flame: &FlameTemplate) -> Vec<u8> {
        let (tx, rx) = channel();
        for _i in 0..self.threads {
            let task: RenderTask = RenderTask::new(&flame, self.threads);
            let tx = tx.clone();
            thread::spawn(move || tx.send(task.render()));
        }

        let mut histograms = vec![];

        for _i in 0..self.threads {
            let hist : Histogram = rx.recv().unwrap();
            histograms.push(hist);
        }

        let processor = HistogramProcessor::new(flame.render.quality);
        processor.process_to_raw(histograms)
    }
}