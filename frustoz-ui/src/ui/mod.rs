use gtk::{Builder, WindowPosition};
use gtk::GtkWindowExt;
use gtk::prelude::*;

use frustoz_io as io;

use crate::ui::bus::Bus;
use crate::ui::bus::process;
use crate::ui::bus::Update::Bind;
use crate::ui::bus::Update::FlameUpdate;
use crate::ui::bus::Update::Open;

pub mod bus;

mod preview;
mod components;

pub fn build_ui(application: &gtk::Application) {
    let mut bus: Bus = bus::create_bus();

    let glade_src = include_str!("ui.glade");
    let builder = Builder::new_from_string(glade_src);
    let window: gtk::ApplicationWindow = builder.get_object("window").unwrap();
    window.set_application(application);
    window.set_position(WindowPosition::Center);

    let preview = preview::create();

    let window_box: gtk::Box = builder.get_object("window_box").unwrap();
    window_box.pack_start(&preview.draw, true, false, 1);


    let scale_x: gtk::SpinButton = builder.get_object("scale_x").unwrap();

    scale_x.connect_value_changed(clone!(bus => move |spin_button| {
        println!("Changed");
        let value = spin_button.get_value();
        process(&bus, FlameUpdate(value));
    }));

    let open_file_dialog: gtk::FileChooserNative = gtk::FileChooserNative::new(Some("Open"), Some(&window), gtk::FileChooserAction::Open, None, None);
    open_file_dialog.connect_response(clone!(bus => move |dialog, _response| {
        let path = dialog.get_filename();
        let name = path.as_ref().and_then(|x| x.to_str());
        name.map(| name | {
            println!("Trying to read file \"{}\"", name);
            let flame = io::parser::parse_file(name).into_iter().next();
            process(&bus, Bind(flame.unwrap())); // ToDo: error handling
        });
        dialog.hide();
    }));

    let menu_open: gtk::MenuItem = builder.get_object("menu_open").unwrap();
    menu_open.connect_activate(clone!(bus => move | _ | {
        println! ("Opening file");
        process(&bus, Open());
    }));

    (*bus).borrow_mut().components = Some(bus::Components { open_file_dialog, scale_x });
    (*bus).borrow_mut().preview = Some(preview);
    window.show_all();
}

