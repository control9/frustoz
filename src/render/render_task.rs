use rand;
use rand::Rng;
use render::histogram::Camera;
use render::histogram::canvas::Canvas;
use render::histogram::canvas::HistogramLayer;
use std::sync::mpsc::Sender;
use template::builders;
use template::flame::Flame;
use util::math::RealPoint;

const SKIP_ITERATIONS : u32 = 20;

pub struct RenderTask {
    camera: Camera,
    canvas: Canvas,
    flame: Flame,
    iterations: u32,
    progress_reporter: Sender<u32>,
}

impl RenderTask {
    pub fn new(flame: Flame, iterations: u32, progress_reporter: Sender<u32>) -> Self {
        let camera = builders::camera(&flame.camera);
        let canvas = builders::canvas(&flame.render);

        RenderTask {
            camera,
            canvas,
            flame,
            iterations,
            progress_reporter,
        }
    }

    pub fn render(mut self) -> HistogramLayer {
        let mut rng = rand::thread_rng();
        let mut last_reported_iteration = 0;
        let report_frequency = self.iterations / 100;

        let xstart: f64 = rng.gen_range(0.0, 1.0);
        let ystart: f64 = rng.gen_range(0.0, 1.0);
        let mut point = RealPoint(xstart, ystart);
        let mut color: f64 = rng.gen_range(0.0, 1.0);

        for iteration in 1..self.iterations {
            let transform_seed: f64 = rng.gen_range(0.0, 1.0);
            let transform = self.flame.transforms.get_transformation(transform_seed);

            let (new_point, new_color) = transform.apply(&point, color);
            point = new_point;
            color = new_color;

            if iteration % report_frequency == 0 {
                self.progress_reporter.send(iteration - last_reported_iteration).unwrap();
                last_reported_iteration = iteration;
            }

            if iteration > SKIP_ITERATIONS {
                let camera_coordinates = self.camera.project(&point);
                self.canvas.project_and_update(&camera_coordinates, self.flame.palette.get_color(color));
            }
        }
        self.progress_reporter.send(self.iterations - last_reported_iteration).unwrap();
        self.canvas.extract_data()
    }
}

