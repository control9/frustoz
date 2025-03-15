use crate::model::flame::Flame;

mod barnsley;
pub mod green_palette;
mod sierpinsky;
mod spark;

pub fn sierpinsky() -> Flame {
    sierpinsky::get_flame_model()
}

pub fn barnsley() -> Flame {
    barnsley::get_flame_model()
}

pub fn spark() -> Flame {
    spark::get_flame_model()
}
