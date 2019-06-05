use rand::prelude::*;
use super::histogram::Camera;
use super::Progress;
use super::Histogram;
use super::ProgressReporter;
use crate::model::builders;
use crate::model::flame::Flame;
use crate::util::math::RealPoint;

const SKIP_ITERATIONS : u32 = 20;

pub struct RenderTask<T: ProgressReporter + Sized> {
    camera: Camera,
    canvas: Histogram,
    flame: Flame,
    iterations: u32,
    id: usize,
    progress_reporter: T,
}

impl <T: ProgressReporter + Sized> RenderTask<T> {
    pub fn new(flame: Flame, iterations: u32, id: usize, progress_reporter: T) -> Self {
        let camera = builders::camera(&flame.camera);
        let canvas = builders::histogram(&flame.render, flame.filter.width);

        RenderTask {
            camera,
            canvas,
            flame,
            iterations,
            id,
            progress_reporter,
        }
    }

    pub fn render(mut self) -> Histogram {
        let mut rng = thread_rng();
        let report_frequency = self.iterations / 100;

        let xstart: f64 = rng.gen_range(0.0, 1.0);
        let ystart: f64 = rng.gen_range(0.0, 1.0);
        let mut point = RealPoint(xstart, ystart);
        let mut color: f64 = rng.gen_range(0.0, 1.0);

        let mut progress = Progress(0, self.id);

        for iteration in 0..self.iterations {
            let transform_seed: f64 = rng.gen_range(0.0, 1.0);
            let transform = self.flame.transforms.get_transformation(transform_seed);

            let (new_point, new_color) = transform.apply(&point, color, &mut rng);
            point = new_point;
            color = new_color;
            progress.0 += 1;

            if progress.0 % report_frequency == 0 {
                self.progress_reporter.report(progress);
                progress.0 = 0;
            }

            if iteration > SKIP_ITERATIONS {
                let camera_coordinates = self.camera.project(&point);
                self.canvas.project_and_update(&camera_coordinates, self.flame.palette.get_color(color));
            }
        }
        self.progress_reporter.report(progress);
        self.canvas
    }
}

