use gtk::{DrawingArea, WidgetExt, Inhibit};

use gdk::ContextExt;
use crate::ui::UIState;
use std::sync::{Arc, Mutex};


pub fn create( state: &Arc<Mutex<UIState>>) -> DrawingArea {
    let da = DrawingArea::new();
    da.connect_draw(clone!( state => move |x, c| {
        let st = &mut state.lock().unwrap();
        draw(x, c, st);
        Inhibit(false)
    }));
    da
}

 fn draw(draw: &gtk::DrawingArea, c: &cairo::Context, state: &mut UIState) {
    draw.set_size_request(1024, 768);
    state.raw.as_ref().map(|buf| {
        c.set_source_pixbuf(&buf, 0f64, 0f64);
        c.paint();
    });
}
