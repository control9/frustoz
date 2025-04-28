use futures::future::join_all;
use image::ImageEncoder;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::Relaxed;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot::error::TryRecvError;
use tokio::sync::oneshot::Receiver;
use tokio::task::{spawn_blocking, JoinHandle};
use tokio::time::sleep;
use tokio_with_wasm::alias as tokio;

use web_time::Duration;
use web_time::Instant;

use crate::model::builders;
use crate::model::flame::Flame;
use crate::render::progressive_render_task::ProgressiveRenderTask;
use crate::render::{Canvas};

pub struct Snapshot {
    pub image_data: Vec<u8>,
    pub frame_time: f64,
    pub steps: u64,
    pub complete: bool,
}

pub enum  Command {
    CANCEL
}

pub(crate) struct CanvasState {
    pub(crate) canvas: Canvas,
    pub(crate) steps: AtomicU64, // TODO: calculate on task level instead to avoid dependency on 64bit atomics 
}

const MIN_FRAME_DURATION: Duration = Duration::from_millis(5);

pub async fn render(
    flame: Flame,
    max_steps: u64,
    frame_delta: Duration,
    threads: usize,
    snapshot_tx: Sender<Snapshot>,
    mut command_rx: Receiver<Command>
) {
    info!("Beginning rendering");
    let start_time = Instant::now();

    let fl = flame.clone();
    let processor = builders::histogram_processor(&fl);

    // create tasks;
    let fl = flame.clone();
    let canvas_state = CanvasState { canvas: builders::canvas(&flame.render, flame.filter.width), steps: AtomicU64::new(0)};
    let canvas = Arc::new(canvas_state);
    let tasks = create_tasks(&fl, threads, canvas.clone());
    info!("Created task definitions");
    let handles : Vec<JoinHandle<()>> = tasks.into_iter().map(|mut t| {
        tokio::task::spawn(async move {t.render().await})
    }).collect();
    info!("Spawned tasks");

    let mut frame_start = Instant::now();
    let mut adjusted_delta = frame_delta.clone();

    let mut total_iterations = 0;
    while total_iterations < max_steps {
        sleep(adjusted_delta).await;

        let pr = processor.clone();
        let cc = canvas.clone();

        let raw = spawn_blocking(move || pr.process_to_raw_single(&cc.canvas))
            .await
            .expect("Histogram merging failed");

        let image_data = encode_image(flame.render.width, flame.render.height, raw);
        let actual_passed = frame_start.elapsed();
        let complete = total_iterations >= max_steps;
        snapshot_tx
            .send(Snapshot {
                image_data,
                frame_time: actual_passed.as_secs_f64(),
                steps: total_iterations,
                complete,
            })
            .await
            .expect("Failed to send snapshot");

        match command_rx.try_recv() {
            Err(TryRecvError::Empty) => (),
            _ => {
                handles.iter().for_each(|h| h.abort());
                break;
            }
        }

        if actual_passed > frame_delta {
            let adjustment = actual_passed - frame_delta;
            if frame_delta > (adjustment + MIN_FRAME_DURATION) {
                adjusted_delta = frame_delta - adjustment;
            } else {
                adjusted_delta = frame_delta;
            }
        } else {
            adjusted_delta = frame_delta;
        }
        frame_start = Instant::now();
        total_iterations = canvas.steps.load(Relaxed);
    }

    join_all(handles).await;

    info!(
        "Rendering took {} seconds",
        start_time.elapsed().as_secs_f64()
    )
}

fn encode_image(width: u32, height: u32, raw: Vec<u8>) -> Vec<u8> {
    let mut image_data: Vec<u8> = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut image_data);
    let result = encoder.write_image(
        raw.as_slice(),
        width,
        height,
        image::ExtendedColorType::Rgb8,
    );
    result.expect("Failed to encode image");
    image_data
}

fn create_tasks(
    flame: &Flame,
    threads: usize,
    canvas: Arc<CanvasState>
) -> Vec<ProgressiveRenderTask> {
    (0..threads)
        .into_iter()
        .map(|id|  ProgressiveRenderTask::new(flame.clone(), canvas.clone(), id))
        .collect()
}
