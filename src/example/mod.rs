use template::flame::Flame;

mod sierpinsky;
mod barnsley;
mod spark;
pub mod green_palette;

pub fn sierpinsky() -> Flame {
    sierpinsky::get_flame_template()
}

pub fn barnsley() -> Flame {
    barnsley::get_flame_template()
}

pub fn spark() -> Flame {
    spark::get_flame_template()
}