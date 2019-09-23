use std::cell::RefCell;
use std::rc::Rc;

use gdk_pixbuf::{Colorspace, Pixbuf};
use glib::Bytes;
use gtk::{Continue, SpinButtonExt, WidgetExt};
use gtk::NativeDialogExt;

use frustoz_core::model::flame::Flame;

use crate::render::render;
use crate::ui::preview::Preview;
use super::widgets::Widgets;

pub struct BusImpl {
    pub flame: Option<Flame>,
    pub widgets: Widgets,
    pub preview: Preview,
    pub binding: bool,
}

pub enum Update {
    Bind(Flame),
    FlameUpdate(f64),
    Render(bool),
    Redraw(Vec<u8>),
    Open(),
}

pub type Bus = Rc<RefCell<BusImpl>>;

pub fn process(bus_ref: &Bus, event: Update) {
    let bus = (*bus_ref).clone();
    let mut itself = (*bus).borrow_mut();

    match event {
        Update::Bind(f) => {
            println!("Bind");
            itself.binding = true;
            itself.flame = Some(f.clone());
            itself.widgets.bind(&f, &bus);
        }
        Update::FlameUpdate(f) => {
            println!("Update");
            itself.flame.as_mut().unwrap().camera.scale_x = f; // Handle None
            gtk::idle_add(clone!(bus => move || {
                process(&bus, Update::Render(false));
                Continue(false)
            }));
        }
        Update::Open() => {
            println!("Open");
            itself.widgets.open_file_dialog.show();
        }
        Update::Render(force) => {
            println!("Render");
            if force {
                itself.binding = false;
            }
            if !itself.binding {
                println!("Actually rendering");
                let f = itself.flame.clone();
                gtk::idle_add(clone!(bus => move || {
                    render(&bus, &f);
                    Continue(false)
                }));
            }
        }
        Update::Redraw(raw) => {
            println!("Redraw");
            let pixbuf = {
                let raw_bytes = Bytes::from(&raw);
                Pixbuf::new_from_bytes(&raw_bytes, Colorspace::Rgb, false, 8, 1024, 768, 3 * 1024)
            };


            let buf = itself.preview.pix_buf.clone();
            let mut buffer = (*buf).borrow_mut();
            *buffer = Some(pixbuf);
            itself.preview.draw.queue_draw();
        }
    }
}


pub fn create_bus(widgets: &Widgets, preview: Preview) -> Bus {
    let bus_impl = BusImpl {
        flame: None,
        widgets: widgets.clone(),
        preview,
        binding: false,
    };
    Rc::new(RefCell::new(bus_impl))
}