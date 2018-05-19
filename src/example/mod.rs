use transforms::Transformation;

mod sierpinsky;
mod barn;

pub struct ExampleTransformations(Vec<Transformation>);

impl ExampleTransformations {
    pub fn new() -> Self {
        ExampleTransformations(vec![
            Transformation::new(sierpinsky::get_transform()),
            Transformation::new(barn::get_transform()),
        ])
    }

    pub fn sierpinski(&self) -> &Transformation {
        &self.0[0]
    }

    pub fn barn(&self) -> &Transformation {
        &self.0[1]
    }
}