use std::sync::Arc;
use std::sync::atomic::Ordering::Relaxed;
use crate::model::builders;
use crate::model::flame::Flame;
use crate::render::canvas::Camera;
use crate::render::{Canvas};
use crate::util::math::RealPoint;
use rand::{rng, Rng};
use tokio_with_wasm::alias as tokio;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::yield_now;
use crate::render::progressive_renderer::CanvasState;

pub struct ProgressiveRenderTask {
    camera: Camera,
    canvas: Arc<CanvasState>,
    flame: Flame,
    id: usize
}

const SKIP_ITERATIONS: u64 = 20;

impl ProgressiveRenderTask {
    pub(crate) fn new(
        flame: Flame,
        canvas: Arc<CanvasState>,
        id: usize,
    ) -> Self {
        let camera = builders::camera(&flame.camera);
        ProgressiveRenderTask {
            camera,
            canvas,
            flame,
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
                self.canvas.canvas
                    .project_and_update(&camera_coordinates, self.flame.palette.get_color(color));
            }
            _ = self.canvas.steps.fetch_add(1, Relaxed);
            yield_now().await;
        }
    }
}
