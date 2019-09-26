use std::cell::RefCell;
use std::rc::Rc;

use gdk::ContextExt;
use gdk_pixbuf::{Colorspace, Pixbuf};
use glib::Bytes;
use gtk::{DrawingArea, Inhibit, WidgetExt, BoxExt, Builder, false_};
use crate::ui::bus::{Subscriber, Update};

pub struct Preview {
    pub draw: DrawingArea,
    pub pix_buf: Rc<RefCell<Option<Pixbuf>>>,
}

impl Preview {
    pub fn new(builder: &Builder) -> Self {
        let drawing_area = DrawingArea::new();
        let pix_buf = Rc::new(RefCell::new(None));

        let window_box: gtk::Box = builder.get_object("window_box").unwrap();
        window_box.pack_start(&drawing_area, true, false, 1);

        drawing_area.connect_draw(clone!( pix_buf => move |x, c| {
            draw(x, c, &pix_buf.borrow());
            Inhibit(false)
        }));

        Preview {
            draw: drawing_area,
            pix_buf,
        }
    }

    fn redraw(&self, raw: &Vec<u8>) {
        let pixbuf = {
            let raw_bytes = Bytes::from(raw);
            Pixbuf::new_from_bytes(&raw_bytes, Colorspace::Rgb, false, 8, 1024, 768, 3 * 1024)
        };
        let buf = self.pix_buf.clone();
        let mut buffer = (*buf).borrow_mut();
        *buffer = Some(pixbuf);
        self.draw.queue_draw();
    }
}

fn draw(draw: &gtk::DrawingArea, c: &cairo::Context, buf: &Option<Pixbuf>) {
    draw.set_size_request(1024, 768);
    buf.as_ref().map(|buf| {
        c.set_source_pixbuf(&buf, 0f64, 0f64);
        c.paint();
    });
}

impl Subscriber for Preview {
    fn accepts(&self, e: &Update) -> bool {
        match e {
            Update::Redraw(_) => true,
            _ => false
        }
    }

    fn process(&mut self, e: &Update) {
        match e {
            Update::Redraw(raw) => self.redraw(raw),
            _ => {}
        }
    }
}
