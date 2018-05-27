use rand;
use rand::Rng;
use render::camera::Camera;
use render::canvas::Canvas;
use render::canvas::HistogramLayer;
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
    iteration: u32,
    skip_iterations: u32,
    point: RealPoint,
    color: f64,
}

impl RenderTask {
    pub fn new(template: &FlameTemplate, threads: u32) -> Self {
        let mut rng = rand::thread_rng();

        let camera = builders::camera(&template.camera);
        let canvas = builders::canvas(&template.render);
        let variations = builders::transform_system(&template.transforms);
        let palette : Palette = (&template.palette).clone();

        let iterations = builders::iterations(&template.render) / threads;
        let skip_iterations = &template.render.skip_iterations;

        let xstart: f64 = rng.gen_range(0.0, 1.0);
        let ystart: f64 = rng.gen_range(0.0, 1.0);
        let point = RealPoint(xstart, ystart);
        let color : f64 = rng.gen_range(0.0, 1.0);

        RenderTask {
            camera, canvas, variations, palette, iterations, iteration: 0, skip_iterations: *skip_iterations, point, color
        }

    }

    pub fn render(mut self) -> HistogramLayer {
        let mut rng = rand::thread_rng();
        for iteration in 1..self.iterations {
            self.iteration = iteration;
            let transform_seed: f64 = rng.gen_range(0.0, 1.0);
            let transform = self.variations.get_transformation(transform_seed);

            let (new_point, new_color) = transform.apply(&self.point, self.color);
            self.point = new_point;
            self.color = new_color;

            if iteration > self.skip_iterations {
                let camera_coordinates = self.camera.project(&self.point);
                self.canvas.project_and_update(&camera_coordinates, self.palette.get_color(self.color));
            }
        }

        self.canvas.extract_data()
    }
}

