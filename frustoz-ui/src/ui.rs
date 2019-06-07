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

    let example_selector = gtk::ComboBoxText::new();
    example_selector.append_text(SPARK_STR);
    example_selector.append_text(SIERPINSKY_STR);
    example_selector.append_text(BARNSLEY_STR);

    let dr = drawing.clone();
    {
        let mut sta = Arc::clone(&state);
        example_selector.connect_changed(move |x| {
            let st = &mut sta.lock().unwrap();
            on_select(x, st, dr.clone());
        });
    }
    bx.add(&example_selector);
    bx.add(&drawing);
    window.show_all();
}


fn render_flame(flame: &Flame) -> Pixbuf {
    let threads = (num_cpus::get() as u32 - crate::PRESERVE_CPUS).max(1);
    let renderer = render::multithreaded_renderer::Renderer { threads };
    let mut f = flame.clone();
    f.render.width = 1024;
    f.render.height = 768;
    f.render.quality = 100;

    let raw = renderer.render::<render::NoOpReporter>(f);
    let raw_bytes = Bytes::from(&raw);
    Pixbuf::new_from_bytes(&raw_bytes, Colorspace::Rgb, false, 8, 1024, 768, 3 * 1024)
}

fn init_state() -> UIState {
    UIState { flame: None, raw: None }
}

fn on_select(example_selector: &gtk::ComboBoxText, state: &mut UIState, dr: gtk::DrawingArea) {
    use Example::*;
    let id = example_selector.get_active_text().map(|i| i.to_string());
    let example = id.as_ref().map(|x| &**x) // Converting Option<String> to Option<&str> never fails to amuse me
        .and_then(Example::get_example);
    state.flame = example.map(|x| match  x {
        Spark => frustoz_core::example::spark(),
        Sierpinsky => frustoz_core::example::sierpinsky(),
        Barnsley => frustoz_core::example::barnsley(),
    });
    state.raw = state.flame.as_ref().map(|flame| render_flame(flame));
    dr.queue_draw();
}

pub fn draw(draw: &gtk::DrawingArea, c: &cairo::Context, state: &mut UIState) {
    draw.set_size_request(1024, 768);
    state.raw.as_ref().map(|buf| {
        c.set_source_pixbuf(&buf, 0f64, 0f64);
        c.paint();
    });
}

enum Example {
    Spark,
    Sierpinsky,
    Barnsley,
}

const SPARK_STR: &str = "spark";
const SIERPINSKY_STR: &str = "sierpinsky";
const BARNSLEY_STR: &str = "barnsley";


impl Example {

    fn get_text(&self) -> &str {
        use Example::*;
        match self {
            Spark => SPARK_STR,
            Sierpinsky => SIERPINSKY_STR,
            Barnsley => BARNSLEY_STR,
        }
    }

    fn get_example(id: &str) -> Option<Example> {
        use Example::*;
        match id {
            SPARK_STR => Some(Spark),
            SIERPINSKY_STR => Some(Sierpinsky),
            BARNSLEY_STR => Some(Barnsley),
            _ => None,
        }
    }
}
