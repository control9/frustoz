use frustoz_core::example::*;
use frustoz_core::model::flame::Flame;

pub const SPARK_STR: &str = "spark";
pub const SIERPINSKY_STR: &str = "sierpinsky";
pub const BARNSLEY_STR: &str = "barnsley";


pub fn get_example(id: &str) -> Option<Flame> {
    match id {
        SPARK_STR => Some(spark()),
        SIERPINSKY_STR => Some(sierpinsky()),
        BARNSLEY_STR => Some(barnsley()),
        _ => None,
    }
}
