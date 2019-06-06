use core::borrow::BorrowMut;
use std::sync::{Arc, Mutex};

use cairo;
use gdk::prelude::*;
use gdk_pixbuf::{Colorspace, Pixbuf};
use glib::Bytes;
use gtk::{WidgetExt, WindowPosition};
use gtk::prelude::*;

use frustoz_core::model::flame::Flame;
use frustoz_core::render;
use num_cpus;

pub struct UIState {
    flame: Option<Flame>,
    raw: Option<Pixbuf>,
}

pub fn build_ui(application: &gtk::Application) {
    let state: Arc<Mutex<UIState>> = Arc::new(Mutex::new(init_state()));
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_position(WindowPosition::Center);
    window.set_default_size(350, 70);

    let bx = gtk::FlowBox::new();
    window.add(&bx);

    let drawing = gtk::DrawingArea::new();
    {
        let mut sta = Arc::clone(&state);
        drawing.connect_draw(move |x, c| {
            let st = &mut sta.lock().unwrap();
            draw(x, c, st);
            Inhibit(false)
        });
    }

    let button = gtk::Button::new_with_label("Click me!");
    let dr = drawing.clone();
    {
        let mut sta = Arc::clone(&state);
        button.connect_clicked(move |x| {
            let st = &mut sta.lock().unwrap();
            on_click(x, st, dr.clone());
        });
    }
    bx.add(&button);
    bx.add(&drawing);
    window.show_all();
}


fn render_flame(flame: &Flame) -> Pixbuf {
    let threads = (num_cpus::get() as u32 - crate::PRESERVE_CPUS).max(1);
    let renderer = render::multithreaded_renderer::Renderer { threads };

    let raw = renderer.render::<render::NoOpReporter>(flame.clone());
    let raw_bytes = Bytes::from(&raw);
    Pixbuf::new_from_bytes(&raw_bytes, Colorspace::Rgb, false, 8, 1024, 768, 3 * 1024)
}

fn init_state() -> UIState {
    UIState { flame: None, raw: None }
}

fn on_click(button: &gtk::Button, state: &mut UIState, dr: gtk::DrawingArea) {
    state.flame = Some(frustoz_core::example::spark());
    state.raw = Some(render_flame(state.flame.as_ref().unwrap()));
    dr.queue_draw();
}

pub fn draw(draw: &gtk::DrawingArea, c: &cairo::Context, state: &mut UIState) {
    draw.set_size_request(1024, 768);
    state.raw.as_ref().map(|buf| {
        c.set_source_pixbuf(&buf, 0f64, 0f64);
        c.paint();
    });
}