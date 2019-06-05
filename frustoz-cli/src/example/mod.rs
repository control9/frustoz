use model::flame::Flame;

mod sierpinsky;
mod barnsley;
mod spark;
pub mod green_palette;

pub fn sierpinsky() -> Flame {
    sierpinsky::get_flame_model()
}

pub fn barnsley() -> Flame {
    barnsley::get_flame_model()
}

pub fn spark() -> Flame {
    spark::get_flame_model()
}