use std::thread::spawn;

use gdk_pixbuf::{Colorspace, Pixbuf};
use glib::{Bytes, Continue, MainContext};
use gtk::{DrawingArea, WidgetExt};
use num_cpus;

use frustoz_core::model::flame::Flame;
use frustoz_core::render;

use crate::ui::state::State;
use crate::ui::bus::Bus;
use crate::ui::bus::process;
use crate::ui::bus::Update::Redraw;

pub fn render_bus(bus: &Bus, flame: &Option<Flame>) {

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

pub fn render(state: &State) {
    let (tx, rx) = MainContext::channel::<Option<Vec<u8>>>(glib::PRIORITY_DEFAULT);

    let flame = state.lock().unwrap().flame.as_ref().map(|f| f.clone());

    spawn(
        move || {
            tx.send(flame.as_ref().map(|f| render_flame(f)));
        }
    );

    rx.attach(None, clone!(state => move |raw| {
        let buf = raw.map( |raw| {
            let raw_bytes = Bytes::from(&raw);
            Pixbuf::new_from_bytes(&raw_bytes, Colorspace::Rgb, false, 8, 1024, 768, 3 * 1024)
        });

        let state = &mut state.lock().unwrap();
        state.raw = buf;
        info!("Completed render");
        state.components.as_ref().map(|c| c.drawing.queue_draw());
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