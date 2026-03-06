use crate::model::builders;
use crate::model::flame::Flame;
use crate::render::canvas::Camera;
use crate::render::progressive_renderer::{Command, TaskUpdateRequest};
use crate::render::Canvas;
use crate::util::math::RealPoint;
use rand::{rng, Rng};
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::TryRecvError;
use tokio::task::yield_now;
use tokio_with_wasm::alias as tokio;

pub struct ProgressiveRenderTask {
    camera: Camera,
    canvas: Arc<Canvas>,
    flame: Flame,
    rx: broadcast::Receiver<TaskUpdateRequest>,
    id: usize,
}

const SKIP_ITERATIONS: u64 = 20;

impl ProgressiveRenderTask {
    pub(crate) fn new(
        flame: Flame,
        canvas: Arc<Canvas>,
        rx: broadcast::Receiver<TaskUpdateRequest>,
        id: usize,
    ) -> Self {
        let camera = builders::camera(&flame.camera);
        ProgressiveRenderTask {
            camera,
            canvas,
            flame,
            rx,
            id,
        }
    }

    pub(crate) async fn render(&mut self) -> () {
        let xstart: f64 = rng().random_range(0.0..1.0);
        let ystart: f64 = rng().random_range(0.0..1.0);
        let mut point = RealPoint(xstart, ystart);
        let mut color: f64 = rng().random_range(0.0..1.0);

        let mut iteration = 0;
        info!("Task {} started", self.id);
        loop {
            let transform = {
                let transform_seed: f64 = rng().random_range(0.0..1.0);
                self.flame.transforms.get_transformation(transform_seed)
            };

            let (new_point, new_color) = transform.apply(&point, color, &mut rng());
            point = new_point;
            color = new_color;
            iteration += 1;

            if iteration > SKIP_ITERATIONS {
                let camera_coordinates = self.camera.project(&point);
                self.canvas
                    .project_and_update(&camera_coordinates, self.flame.palette.get_color(color));
            }
            
            match (self.rx.try_recv()) {
                Ok(TaskUpdateRequest{command: Command::CANCEL, ..})  | Err(TryRecvError::Closed) => break,
                Ok(TaskUpdateRequest{command: Command::UPDATE, steps_sender}) => 
                    steps_sender.send(iteration).await.expect("Failed to send update"),
                _ => {}
            }

            yield_now().await;
        }
    }
}
