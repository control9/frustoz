use gtk::{DrawingArea, WidgetExt, Inhibit};

use gdk::ContextExt;
use super::state::State;
use std::sync::{Arc, Mutex};
use crate::ui::bus::Bus;
use gdk_pixbuf::Pixbuf;
use std::cell::RefCell;
use std::rc::Rc;


pub fn create( state: &State) -> DrawingArea {
    let da = DrawingArea::new();
    da.connect_draw(clone!( state => move |x, c| {
        draw(x, c, &state);
        Inhibit(false)
    }));
    da
}

 fn draw(draw: &gtk::DrawingArea, c: &cairo::Context, state: &State) {
    draw.set_size_request(1024, 768);
     let st = &mut state.lock().unwrap();
     st.raw.as_ref().map(|buf| {
        c.set_source_pixbuf(&buf, 0f64, 0f64);
        c.paint();
    });
}
