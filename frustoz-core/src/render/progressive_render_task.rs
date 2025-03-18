use crate::model::builders;
use crate::model::flame::Flame;
use crate::render::histogram::Camera;
use crate::render::progressive_renderer::{SingleThreadSnapshot, TaskCommand};
use crate::render::Histogram;
use crate::util::math::RealPoint;
use rand::{rng, Rng};
use tokio_with_wasm::alias as tokio;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct ProgressiveRenderTask {
    camera: Camera,
    canvas: Histogram,
    flame: Flame,
    id: usize,
    rx: Receiver<TaskCommand>,
    tx: Sender<SingleThreadSnapshot>,
}

const SKIP_ITERATIONS: u64 = 20;

impl ProgressiveRenderTask {
    pub(crate) fn new(
        flame: Flame,
        rx: Receiver<TaskCommand>,
        id: usize,
        tx: Sender<SingleThreadSnapshot>,
    ) -> Self {
        let camera = builders::camera(&flame.camera);
        let canvas = builders::histogram(&flame.render, flame.filter.width);
        ProgressiveRenderTask {
            camera,
            canvas,
            flame,
            id,
            rx,
            tx,
        }
    }

    pub(crate) fn render(&mut self) {
        let mut rng = rng();
        let xstart: f64 = rng.random_range(0.0..1.0);
        let ystart: f64 = rng.random_range(0.0..1.0);
        let mut point = RealPoint(xstart, ystart);
        let mut color: f64 = rng.random_range(0.0..1.0);

        let mut complete = false;
        let mut iteration = 0;
        info!("Task {} started", self.id);
        while !complete {
            let transform_seed: f64 = rng.random_range(0.0..1.0);
            let transform = self.flame.transforms.get_transformation(transform_seed);

            let (new_point, new_color) = transform.apply(&point, color, &mut rng);
            point = new_point;
            color = new_color;
            iteration += 1;

            if iteration > SKIP_ITERATIONS {
                let camera_coordinates = self.camera.project(&point);
                self.canvas
                    .project_and_update(&camera_coordinates, self.flame.palette.get_color(color));
            }

            match self.rx.try_recv() {
                Ok(TaskCommand::FrameExpected) => {
                    let canv = self.canvas.clone();
                    let snapshot = SingleThreadSnapshot {
                        histogram: canv,
                        steps: iteration,
                    };
                    self.tx.blocking_send(snapshot).unwrap();
                }
                Ok(TaskCommand::Completed) => {
                    complete = true;
                }
                Err(TryRecvError::Disconnected) => {
                    error!("Sender for TaskCommand disconnected before sending Completed");
                    complete = true;
                }
                Err(TryRecvError::Empty) => {} // Nothing to do
            }
        }
    }
}
