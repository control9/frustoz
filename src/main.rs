extern crate image;
extern crate rand;

pub mod camera;
pub mod canvas;
pub mod coordinates;
pub mod example;
pub mod math;
pub mod template;
pub mod transforms;
pub mod renderer;
pub mod builders;
pub mod output;

fn main() {
    let template = example::barnsley();
    let raw = renderer::render(&template);
    output::write("fractal.png",raw, &template.render);
}


