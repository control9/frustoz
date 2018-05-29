use rand;
use rand::Rng;
use render::camera::Camera;
use render::canvas::Canvas;
use render::canvas::HistogramLayer;
use std::sync::mpsc::Sender;
use template::builders;
use template::flame_template::FlameTemplate;
use template::palette::Palette;
use transforms::TransformSystem;
use util::math::RealPoint;

pub struct RenderTask {
    camera: Camera,
    canvas: Canvas,
    variations: TransformSystem,
    palette: Palette,
    iterations: u32,
    skip_iterations: u32,
    progress_reporter: Sender<u32>,
    point: RealPoint,
    color: f64,
}

impl RenderTask {
    pub fn new(template: &FlameTemplate, iterations: u32, progress_reporter: Sender<u32>) -> Self {
        let mut rng = rand::thread_rng();

        let camera = builders::camera(&template.camera);
        let canvas = builders::canvas(&template.render);
        let variations = builders::transform_system(&template.transforms);
        let palette: Palette = (&template.palette).clone();

        let skip_iterations = template.render.skip_iterations;

        let xstart: f64 = rng.gen_range(0.0, 1.0);
        let ystart: f64 = rng.gen_range(0.0, 1.0);
        let point = RealPoint(xstart, ystart);
        let color: f64 = rng.gen_range(0.0, 1.0);

        RenderTask {
            camera,
            canvas,
            variations,
            palette,
            iterations,
            skip_iterations,
            progress_reporter,
            point,
            color,
        }
    }

    pub fn render(mut self) -> HistogramLayer {
        let mut rng = rand::thread_rng();
        let mut last_reported_iteration = 0;
        let report_frequency = self.iterations / 100;

        for iteration in 1..self.iterations {
            let transform_seed: f64 = rng.gen_range(0.0, 1.0);
            let transform = self.variations.get_transformation(transform_seed);

            let (new_point, new_color) = transform.apply(&self.point, self.color);
            self.point = new_point;
            self.color = new_color;

            if iteration % report_frequency == 0 {
                self.progress_reporter.send(iteration - last_reported_iteration).unwrap();
                last_reported_iteration = iteration;
            }

            if iteration > self.skip_iterations {
                let camera_coordinates = self.camera.project(&self.point);
                self.canvas.project_and_update(&camera_coordinates, self.palette.get_color(self.color));
            }
        }
        self.progress_reporter.send(self.iterations - last_reported_iteration).unwrap();
        self.canvas.extract_data()
    }
}

