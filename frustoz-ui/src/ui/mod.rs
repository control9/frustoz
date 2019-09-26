use gtk::{Builder, WindowPosition};
use gtk::GtkWindowExt;
use gtk::prelude::*;

use bus::Bus;

pub mod bus;

mod preview;
mod widgets;

pub fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("ui.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: gtk::ApplicationWindow = builder.get_object("window").unwrap();
    window.set_application(application);
    window.set_position(WindowPosition::Center);

    let preview = preview::create();
    let widgets = widgets::create(builder, &window, &preview);
    let bus: Bus = bus::create_bus(&widgets, preview);
    widgets.connect(&bus);

    window.show_all();
}
