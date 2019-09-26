use gtk::Continue;

use frustoz_core::model::flame::Flame;

use crate::ui::bus::{Bus, process, Subscriber, Event, FlameUpdate};

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

    fn update(&mut self, update: &FlameUpdate) -> () {
        if let Some(flame) = self.flame.as_mut() {

            update_flame(flame, update);

            let bus = self.bus.clone();
            gtk::idle_add(move || {
                process(&bus, Event::SuggestRender());
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
                process(&bus, Event::DoRender(&f));
                Continue(false)
            });
        }
    }
}

fn update_flame(flame : &mut Flame, update: &FlameUpdate) {
    match *update {
        FlameUpdate::ScaleX(val) => flame.camera.scale_x = val,
        FlameUpdate::ScaleY(val) =>  flame.camera.scale_y = val,
        FlameUpdate::OriginX(val) =>  flame.camera.origin.0 = val,
        FlameUpdate::OriginY(val) =>  flame.camera.origin.1 = val,
    }
}

impl Subscriber for Model {
    fn accepts(&self, e: &Event) -> bool {
        match e {
            Event::PreBind(_) => true,
            Event::PostBind() => true,
            Event::SuggestRender() => true,
            Event::Edit(_) => true,
            _ => false
        }
    }

    fn process(&mut self, e: &Event) {
        match e {
            Event::PreBind(f) => self.pre_bind(f),
            Event::PostBind() => {
                self.rebinding = false;
                self.suggest_render();
            },
            Event::SuggestRender() => self.suggest_render(),
            Event::Edit(update) => self.update(update),
            _ => {}
        }
    }
}
