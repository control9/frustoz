use template::flame_template::FlameTemplate;
use math::RealPoint;
use camera::Camera;
use builders;
use rand;
use rand::Rng;

pub fn render(template: &FlameTemplate) -> Vec<u8> {
    let mut rng = rand::thread_rng();

    let xstart: f64 = rng.gen_range(0.0, 1.0);
    let ystart: f64 = rng.gen_range(0.0, 1.0);

    let mut point = RealPoint(xstart, ystart);
    let mut color : f64 = rng.gen_range(0.0, 1.0);
    let camera = builders::camera(&template.camera);
    let mut canvas = builders::canvas(&template.render);
    let variations = builders::transform_system(&template.transforms);
    let palette = &template.palette;

    let iterations = builders::iterations(&template.render);

    for iteration in 1..iterations {
        let transform_seed: f64 = rng.gen_range(0.0, 1.0);
        let transform = variations.get_transformation(transform_seed);

        let (new_point, new_color) = transform.apply(&point, color);
        point = new_point;
        color = new_color;

        if iteration > template.render.skip_iterations {
            let camera_coordinates = camera.project(&point);
            canvas.project_and_update(&camera_coordinates, palette.get_color(color));
        }
    }

    canvas.extract_raw()
}