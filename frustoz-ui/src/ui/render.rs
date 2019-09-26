use std::thread::spawn;

use glib::{Continue, MainContext};
use num_cpus;

use frustoz_core::model::flame::Flame;
use frustoz_core::render;

use crate::ui::bus::{Bus, Subscriber, Update};
use crate::ui::bus::process;
use crate::ui::bus::Update::Redraw;

#[derive(Clone)]
pub struct RenderClient {
    bus: Bus
}

impl RenderClient {
    pub fn render(&self, flame: &Flame) {
        let (tx, rx) = MainContext::channel::<Option<Vec<u8>>>(glib::PRIORITY_DEFAULT);
        let flame = override_flame_with_preview(flame);
        spawn(
            move || {
                tx.send(Some(render_flame(&flame)));
            }
        );

        let bus = &self.bus;
        rx.attach(None, clone!(bus => move |raw| {
            if let Some(actual_raw) = raw {
                process(&bus, Redraw(actual_raw));
            }
            Continue(false)
        }));
    }

    pub fn new(bus : &Bus) -> Self {
        RenderClient{ bus: bus.clone() }
    }

}

impl Subscriber for RenderClient {
    fn accepts(&self, e: &Update) -> bool {
        match e {
            Update::DoRender(_)  => true,
            _ => false
        }
    }

    fn process(&mut self, e: &Update) {
        match e {
            Update::DoRender(flame)  => self.render(flame),
            _ => {}
        }
    }
}


fn render_flame(flame: &Flame) -> Vec<u8> {
    info!("Started render");
    let flame = override_flame_with_preview(flame);

    let threads = (num_cpus::get() as u32 - crate::PRESERVE_CPUS).max(1);
    let renderer = render::multithreaded_renderer::Renderer { threads };

    renderer.render::<render::NoOpReporter>(flame.clone())
}

fn override_flame_with_preview(flame: &Flame) -> Flame {
    let mut f = flame.clone();
    f.render.width = 1024;
    f.render.height = 768;
    f.render.quality = 100;
    f
}