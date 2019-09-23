use std::thread::spawn;

use glib::{Continue, MainContext};
use num_cpus;

use frustoz_core::model::flame::Flame;
use frustoz_core::render;

use crate::ui::bus::Bus;
use crate::ui::bus::process;
use crate::ui::bus::Update::Redraw;

pub fn render(bus: &Bus, flame: &Option<Flame>) {

    let (tx, rx) = MainContext::channel::<Option<Vec<u8>>>(glib::PRIORITY_DEFAULT);

    spawn(clone!( flame =>
        move || {
            tx.send(flame.as_ref().map(|f| render_flame(f)));
        })
    );

    rx.attach(None, clone!(bus => move |raw| {
        if let Some(actual_raw) = raw {
            process(&bus, Redraw(actual_raw));
        }
        Continue(false)
    }));

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