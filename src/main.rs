extern crate image;
extern crate rand;

pub mod render;
pub mod example;
pub mod template;
pub mod transforms;
pub mod output;
pub mod util;

fn main() {
    let template = example::spark();
    let raw = render::simple_renderer::render(&template);
    output::write("fractal.png",raw, &template.render);
}


