use gtk::{ApplicationWindow, Builder, NativeDialogExt};
use gtk::prelude::*;

use frustoz_core::model::flame::Flame;
use frustoz_io as io;

use super::bus::Bus;
use super::bus::Subscriber;
use super::bus::process;
use super::bus::Update;
use super::bus::Update::*;
use super::preview::Preview;

#[derive(Clone)]
pub struct Widgets {
    bus: Bus,
    menu_open: gtk::MenuItem,
    pub open_file_dialog: gtk::FileChooserNative,
    pub scale_x: gtk::SpinButton,
}

pub fn create(builder: Builder, window: &ApplicationWindow, bus: Bus) -> Widgets {
    let menu_open: gtk::MenuItem = builder.get_object("menu_open").unwrap();
    let open_file_dialog: gtk::FileChooserNative = gtk::FileChooserNative::new(Some("Open"), Some(window), gtk::FileChooserAction::Open, None, None);
    let scale_x: gtk::SpinButton = builder.get_object("scale_x").unwrap();

    window.show_all();

    Widgets {
        bus,
        menu_open,
        open_file_dialog,
        scale_x,
    }
}

impl Widgets {
    fn bind(&self, flame: &Flame) {
        let f = flame.clone();
        let itself = self.clone();
        let bus = itself.bus.clone();

        gtk::idle_add(move || {
            itself.scale_x.set_value(f.camera.scale_x);
            Continue(false)
        });

        gtk::idle_add(clone!(bus => move || {
                    process(&bus, PostBind()); // ToDo: error handling
                    Continue(false)
        }));
    }

    fn open(&self) {
        self.open_file_dialog.show();
    }

    pub fn connect(&self, bus: &Bus) {
        self.scale_x.connect_value_changed(clone!(bus => move |spin_button| {
            let value = spin_button.get_value();
            process(&bus, FlameUpdate(value));
        }));

        self.open_file_dialog.connect_response(clone!(bus => move |dialog, _response| {
            let path = dialog.get_filename();
            let name = path.as_ref().and_then(|x| x.to_str());
            name.map(| name | {
                println!("Trying to read file \"{}\"", name);
                let flame = io::parser::parse_file(name).into_iter().next();
                let f = flame.unwrap();

                gtk::idle_add(clone!(bus, f => move || {
                    process(&bus, PreBind(&f));
                    Continue(false)
                }));


                gtk::idle_add(clone!(bus, f => move || {
                    process(&bus, Bind(&f));
                    Continue(false)
                }));

            });
            dialog.hide();
        }));

        self.menu_open.connect_activate(clone!(bus => move | _ | {
            println! ("Opening file");
            process(&bus, Open());
        }));
    }
}

impl Subscriber for Widgets {

    fn accepts(&self, e: &Update) -> bool{
        match e {
            Bind(_) => true,
            Open() => true,
            _ => false
        }
    }

    fn process(&mut self, e: &Update) {
        match e {
            Bind(flame) => {
                self.bind(flame);
            }
            Open() => {
                self.open();
            }
            _ => {}
        }
    }
}
