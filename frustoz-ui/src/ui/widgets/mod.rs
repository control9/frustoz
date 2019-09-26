use gtk::{ApplicationWindow, Builder, NativeDialogExt};
use gtk::prelude::*;

use frustoz_core::model::flame::Flame;
use frustoz_io as io;

use super::bus::Bus;
use super::bus::Subscriber;
use super::bus::process;
use super::bus::Event;
use super::bus::Event::*;
use super::bus::FlameUpdate;
use super::preview::Preview;
use camera::CameraWidgets;

mod camera;

#[derive(Clone)]
pub struct Widgets {
    bus: Bus,
    menu_open: gtk::MenuItem,
    open_file_dialog: gtk::FileChooserNative,
    camera: CameraWidgets,
}

impl Widgets {
    pub fn new(builder: &Builder, window: &ApplicationWindow, bus: &Bus) -> Self {
        let menu_open: gtk::MenuItem = builder.get_object("menu_open").unwrap();
        let open_file_dialog: gtk::FileChooserNative = gtk::FileChooserNative::new(Some("Open"), Some(window), gtk::FileChooserAction::Open, None, None);
        let camera = CameraWidgets::new(builder, bus);

        window.show_all();

        let widgets = Widgets {
            bus: bus.clone(),
            menu_open,
            open_file_dialog,
            camera,
        };
        widgets.connect();
        widgets
    }

    fn connect(&self) {
        let bus = &self.bus;

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

    fn bind(&self, flame: &Flame) {
        self.camera.bind(flame);

        let bus = self.bus.clone();
        gtk::timeout_add(100, clone!(bus => move || {
                    process(&bus, PostBind());
                    Continue(false)
        }));
    }

    fn open(&self) {
        self.open_file_dialog.show();
    }
}

impl Subscriber for Widgets {

    fn accepts(&self, e: &Event) -> bool{
        match e {
            Bind(_) => true,
            Open() => true,
            _ => false
        }
    }

    fn process(&mut self, e: &Event) {
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
