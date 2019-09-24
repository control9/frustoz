use gtk::{ApplicationWindow, Builder};
use gtk::prelude::*;

use frustoz_core::model::flame::Flame;
use frustoz_io as io;

use super::bus::Bus;
use super::bus::process;
use super::bus::Update::*;
use super::preview::Preview;

#[derive(Clone)]
pub struct Widgets {
    menu_open: gtk::MenuItem,
    pub open_file_dialog: gtk::FileChooserNative,
    pub scale_x: gtk::SpinButton,
}

pub fn create(builder: Builder, window: &ApplicationWindow, preview: &Preview) -> Widgets {
    let window_box: gtk::Box = builder.get_object("window_box").unwrap();
    window_box.pack_start(&preview.draw, true, false, 1);

    let menu_open: gtk::MenuItem = builder.get_object("menu_open").unwrap();
    let open_file_dialog: gtk::FileChooserNative = gtk::FileChooserNative::new(Some("Open"), Some(window), gtk::FileChooserAction::Open, None, None);
    let scale_x: gtk::SpinButton = builder.get_object("scale_x").unwrap();

    window.show_all();

    Widgets {
        menu_open,
        open_file_dialog,
        scale_x,
    }
}

impl Widgets {
    pub fn bind(&self, flame: &Flame, bus: &Bus) {
        let f = flame.clone();
        let itself = self.clone();
        gtk::idle_add(move || {
            itself.scale_x.set_value(f.camera.scale_x);
            Continue(false)
        });

        gtk::timeout_add(100, clone!(bus => move|| {
                process(&bus, Render(true));
                Continue(false)
        }));
    }

    pub fn connect(&self, bus: &Bus) {
        self.scale_x.connect_value_changed(clone!(bus => move |spin_button| {
            println!("Changed");
            let value = spin_button.get_value();
            process(&bus, FlameUpdate(value));
        }));

        self.open_file_dialog.connect_response(clone!(bus => move |dialog, _response| {
            let path = dialog.get_filename();
            let name = path.as_ref().and_then(|x| x.to_str());
            name.map(| name | {
                println!("Trying to read file \"{}\"", name);
                let flame = io::parser::parse_file(name).into_iter().next();
                process(&bus, Bind(flame.unwrap())); // ToDo: error handling
            });
            dialog.hide();
        }));

        self.menu_open.connect_activate(clone!(bus => move | _ | {
            println! ("Opening file");
            process(&bus, Open());
        }));
    }
}
