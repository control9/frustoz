use std::cmp::max;
use futures::future::join_all;
use image::ImageEncoder;
use tokio_with_wasm::tokio::sync::mpsc::{Receiver, Sender};
use tokio_with_wasm::tokio::sync::mpsc::channel;
use tokio_with_wasm::tokio::task::{JoinHandle, spawn_blocking};
use tokio_with_wasm::tokio::time::sleep;

use web_time::Instant;
use web_time::Duration;

use crate::model::builders;
use crate::model::builders::iterations;
use crate::model::flame::Flame;
use crate::render::{Histogram, split};
use crate::render::progressive_render_task::ProgressiveRenderTask;
use crate::render::progressive_renderer::TaskCommand::{Completed, FrameExpected};

pub struct Snapshot {
    pub image_data: Vec<u8>,
    pub frame_time: f64,
    pub steps: u64,
    pub complete: bool,
}

pub struct SingleThreadSnapshot {
    pub(crate) histogram: Histogram,
    pub(crate) steps: u64,
}

pub enum TaskCommand {
    Completed,
    FrameExpected,
}
const MIN_FRAME_DURATION: Duration = Duration::from_millis(5);

pub async fn render(flame: Flame, max_steps: u64, frame_delta: Duration, threads: usize, snapshot_tx: Sender<Snapshot>) {
    info!("Beginning rendering");
    let start_time = Instant::now();

    let fl = flame.clone();
    let processor = builders::histogram_processor(&fl);

    let (mut tx, mut rx) = channel(1);

    // create tasks;
    let fl = flame.clone();
    let (tasks, fr_txs) = spawn_blocking(
        move || {create_tasks(&fl, threads,  tx)}
    ).await.expect("Tasks should be created");
    info!("Created task definitions");
    tasks.into_iter().for_each(
        |mut t| {
            spawn_blocking(move || { t.render() });
        }
    );
    info!("Spawned tasks");

    let mut frame_start = Instant::now();
    let mut adjusted_delta = frame_delta.clone();

    let mut total_iterations = 0;
    while total_iterations < max_steps {
        sleep(adjusted_delta).await;
        let mut rr = Vec::with_capacity(threads);
        for i in 0..threads {
            rr.push(fr_txs[i].send(FrameExpected));
        }
        join_all(rr).await;

        // receive
        let mut histograms = Vec::with_capacity(threads);
        total_iterations = 0;
        for i in 0..threads {
            let SingleThreadSnapshot{histogram, steps} = rx.recv().await.expect("Task senders closed before ");
            total_iterations += steps;
            histograms.push(histogram);
        }

        let pr = processor.clone();
        let raw = spawn_blocking(
            move || { pr.process_to_raw(histograms) }
        ).await.expect("Histogram merging failed");


        let image_data = encode_image(flame.render.width, flame.render.height, raw);
        let actual_passed = frame_start.elapsed();
        let complete = total_iterations >= max_steps;
        snapshot_tx.send(
            Snapshot{image_data, frame_time: actual_passed.as_secs_f64(), steps: total_iterations, complete}
        ).await.expect("Failed to send snapshot");

        if (actual_passed > frame_delta) {
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
    }
    for i in 0..threads {
        let _r = fr_txs[i].send(Completed).await;
    }
    info!("Rendering took {} seconds", start_time.elapsed().as_secs_f64())
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
    image_data
}

fn create_tasks(flame: &Flame, threads:usize,  shared_tx: Sender<SingleThreadSnapshot>)
    -> (Vec<ProgressiveRenderTask>, Vec<Sender<TaskCommand>>) {
    (0..threads).into_iter()
        .map(| id | (id, flame.clone(), shared_tx.clone()))
        .map(|_curr| (_curr, channel(2)))
        .map(|((id, fl, tx), (fr_tx, fr_rx))|
                 (ProgressiveRenderTask::new(fl, fr_rx, id, tx), fr_tx)
        ).unzip()
}

