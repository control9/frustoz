use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

use gdk_pixbuf::{Colorspace, Pixbuf};
use glib::Bytes;
use gtk::{Continue, SpinButtonExt, WidgetExt};
use gtk::NativeDialogExt;

use frustoz_core::model::flame::Flame;
use Update::Bind;
use Update::Open;

use crate::render::render_bus;
use crate::ui::preview::Preview;

pub struct BusImpl {
    pub flame: Option<Flame>,
    pub components: Option<Components>,
    pub preview: Option<Preview>,
    pub binding: bool,
}

#[derive(Clone)]
pub struct Components {
    pub open_file_dialog: gtk::FileChooserNative,
    pub scale_x: gtk::SpinButton,
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
    let mut bus = (*bus_ref).clone();
    let mut itself = (*bus).borrow_mut();
    match event {
        Update::Bind(f) => {
            println!("Bind");
            itself.binding = true;
            itself.flame = Some(f.clone());
            let sc = itself.components.as_ref().unwrap().scale_x.clone();

            gtk::idle_add(clone!(bus => move || {
                sc.set_value(f.camera.scale_x);
                Continue(false)
            }));
            gtk::timeout_add(100, clone!(bus => move|| {
                process(&bus, Update::Render(true));
                Continue(false)
            }));
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
            itself.components.as_ref().unwrap().open_file_dialog.show();
        }
        Update::Render(force) => {
            println!("Render");
            if force {
                itself.binding = false;
            }
            if !itself.binding {
                let f = itself.flame.clone();
                gtk::idle_add(clone!(bus => move || {
                    render_bus(&bus, &f);
                    Continue(false)
                }));
            }
        }
        Update::Redraw(raw) => {
            println!("Redraw");
            if let Some(preview) = &mut itself.preview {
                println!("Redrawing preview");
                let pixbuf = {
                    let raw_bytes = Bytes::from(&raw);
                    Pixbuf::new_from_bytes(&raw_bytes, Colorspace::Rgb, false, 8, 1024, 768, 3 * 1024)
                };


                let buf = preview.pix_buf.clone();
                let mut buffer = (*buf).borrow_mut();
                *buffer = Some(pixbuf);
                preview.draw.queue_draw();
            }
        }
    }
}


pub fn create_bus() -> Bus {
    let bus = BusImpl {
        flame: None,
        components: None,
        preview: None,
        binding: false,
    };
    Rc::new(RefCell::new(bus))
}