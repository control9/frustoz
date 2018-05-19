use transforms::Transformation;

mod sierpinsky;
mod barnsley;

pub struct ExampleTransformations(Vec<Transformation>);

impl ExampleTransformations {
    pub fn new() -> Self {
        ExampleTransformations(vec![
            Transformation::new(sierpinsky::get_transform()),
            Transformation::new(barnsley::get_transform()),
        ])
    }

    pub fn sierpinski(&self) -> &Transformation {
        &self.0[0]
    }

    pub fn barnsley(&self) -> &Transformation {
        &self.0[1]
    }
}