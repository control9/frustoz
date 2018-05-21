use transforms::TransformSystem;

mod sierpinsky;
mod barnsley;
pub mod green_palette;

pub struct ExampleTransformations(Vec<TransformSystem>);

impl ExampleTransformations {
    pub fn new() -> Self {
        ExampleTransformations(vec![
            TransformSystem::new(sierpinsky::get_transform()),
            TransformSystem::new(barnsley::get_transform()),
        ])
    }

    pub fn sierpinski(&self) -> &TransformSystem {
        &self.0[0]
    }

    pub fn barnsley(&self) -> &TransformSystem {
        &self.0[1]
    }
}