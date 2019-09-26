use gtk::Continue;

use frustoz_core::model::flame::Flame;

use crate::ui::bus::{Bus, process, Subscriber, Update};

pub struct Model {
    bus: Bus,
    flame: Option<Flame>,
    rebinding: bool,
}

impl Model {
    pub fn new(bus: &Bus) -> Self {
        Model {
            bus: bus.clone(),
            flame: None,
            rebinding: false,
        }
    }

    fn pre_bind(&mut self, f: &Flame) -> () {
        self.flame = Some(f.clone());
        self.rebinding = true;
    }

    fn update(&mut self, scale: &f64) -> () {
        if let Some(flame) = self.flame.as_mut() {

            flame.camera.scale_x = *scale;

            let bus = self.bus.clone();
            gtk::idle_add(move || {
                process(&bus, Update::SuggestRender());
                Continue(false)
            });
        }
    }

    fn suggest_render(&mut self) -> () {
        if !self.rebinding {
            println!("Rebinding");
            let f = self.flame.as_ref().unwrap().clone();
            let bus = self.bus.clone();
            gtk::idle_add(move || {
                process(&bus, Update::DoRender(&f));
                Continue(false)
            });
        }
    }
}


impl Subscriber for Model {
    fn accepts(&self, e: &Update) -> bool {
        match e {
            Update::PreBind(_) => true,
            Update::PostBind() => true,
            Update::SuggestRender() => true,
            Update::FlameUpdate(_) => true,
            _ => false
        }
    }

    fn process(&mut self, e: &Update) {
        match e {
            Update::PreBind(f) => self.pre_bind(f),
            Update::PostBind() => self.rebinding = false,
            Update::SuggestRender() => self.suggest_render(),
            Update::FlameUpdate(scale) => self.update(scale),
            _ => {}
        }
    }
}
