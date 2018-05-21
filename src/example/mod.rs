use template::flame_template::FlameTemplate;

//mod sierpinsky;
mod barnsley;
pub mod green_palette;


pub fn barnsley() -> FlameTemplate {
    barnsley::get_flame_template()
}