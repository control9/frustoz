use futures::future::join_all;
use std::sync::Arc;
use tokio::sync::oneshot::error::TryRecvError;
use tokio::sync::{broadcast, mpsc, oneshot};

use tokio::task::{spawn_blocking, JoinHandle};
use tokio::time::sleep;
use tokio_with_wasm::alias as tokio;

use web_time::Duration;
use web_time::Instant;

use crate::model::builders;
use crate::model::flame::Flame;
use crate::render::progressive_render_task::ProgressiveRenderTask;
use crate::render::progressive_renderer::Command::UPDATE;
use crate::render::Canvas;

pub struct Snapshot {
    pub image_data: Vec<u8>,
    pub frame_time: f64,
    pub steps: u64,
    pub complete: bool,
}

#[derive(Clone, Debug)]
pub enum Command {
    CANCEL,
    UPDATE,
}

#[derive(Clone, Debug)]
pub struct TaskUpdateRequest {
    pub(crate) command: Command,
    pub(crate) steps_sender: mpsc::Sender<u64>,
}

const MIN_FRAME_DURATION: Duration = Duration::from_millis(5);

pub async fn render(
    flame: Flame,
    max_steps: u64,
    frame_delta: Duration,
    threads: usize,
    snapshot_tx: mpsc::Sender<Snapshot>,
    mut command_rx: oneshot::Receiver<Command>,
) {
    info!("Beginning rendering");
    let start_time = Instant::now();

    let processor = builders::canvas_processor(&flame);

    // create tasks;
    let fl = flame.clone();
    let canvas = Arc::new(builders::canvas(&flame.render, flame.filter.width));
    let (mut tx, _) = broadcast::channel::<TaskUpdateRequest>(threads);
    let tasks = create_tasks(&fl, threads, canvas.clone(), &mut tx);
    info!("Created task definitions");
    let handles: Vec<JoinHandle<()>> = tasks
        .into_iter()
        .map(|mut t| tokio::task::spawn(async move { t.render().await }))
        .collect();
    info!("Spawned tasks");

    let mut frame_start = Instant::now();
    let mut adjusted_delta = frame_delta.clone();

    let mut total_iterations = 0;
    while total_iterations < max_steps {
        sleep(adjusted_delta).await;

        let pr = processor.clone();
        total_iterations = request_steps(&mut tx, threads).await;

        let raw = {
            let canvas = canvas.clone();
            spawn_blocking(move || pr.process_to_raw_single(canvas.clone()))
                .await
                .expect("Histogram merging failed")
        };

        let actual_passed = frame_start.elapsed();
        let complete = total_iterations >= max_steps;
        snapshot_tx
            .send(Snapshot {
                image_data: raw,
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
    }

    join_all(handles).await;

    info!(
        "Rendering took {} seconds",
        start_time.elapsed().as_secs_f64()
    )
}

async fn request_steps(sender: &mut broadcast::Sender<TaskUpdateRequest>, threads: usize) -> u64 {
    let (tx, mut rx) = mpsc::channel(1);
    let msg = TaskUpdateRequest {
        command: UPDATE,
        steps_sender: tx,
    };
    sender.send(msg).expect("Failed to request update");
    let mut total_steps = 0;
    for _i in 0..threads {
        total_steps += rx.recv().await.expect("Failed to receive update");
    }
    total_steps
}

fn create_tasks(
    flame: &Flame,
    threads: usize,
    canvas: Arc<Canvas>,
    tx: &mut broadcast::Sender<TaskUpdateRequest>,
) -> Vec<ProgressiveRenderTask> {
    (0..threads)
        .into_iter()
        .map(|id| ProgressiveRenderTask::new(flame.clone(), canvas.clone(), tx.subscribe(), id))
        .collect()
}
