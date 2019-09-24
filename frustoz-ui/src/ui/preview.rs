use std::cell::RefCell;
use std::rc::Rc;

use gdk::ContextExt;
use gdk_pixbuf::Pixbuf;
use gtk::{DrawingArea, Inhibit, WidgetExt};

pub struct Preview {
    pub draw: DrawingArea,
    pub pix_buf: Rc<RefCell<Option<Pixbuf>>>,
}

pub fn create() -> Preview {
    let drawing_area = DrawingArea::new();
    let pix_buf = Rc::new(RefCell::new(None));

    drawing_area.connect_draw(clone!( pix_buf => move |x, c| {
        draw(x, c, &pix_buf.borrow());
        Inhibit(false)
    }));

    Preview {
        draw: drawing_area,
        pix_buf,
    }
}

fn draw(draw: &gtk::DrawingArea, c: &cairo::Context, buf: &Option<Pixbuf>) {
    draw.set_size_request(1024, 768);
    buf.as_ref().map(|buf| {
        c.set_source_pixbuf(&buf, 0f64, 0f64);
        c.paint();
    });
}
