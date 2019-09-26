use gtk::{Builder, WindowPosition};
use gtk::GtkWindowExt;
use gtk::prelude::*;

use bus::subscribe;

use bus::Bus;
use crate::ui::render::RenderClient;
use crate::ui::model::Model;
use crate::ui::widgets::Widgets;

pub mod bus;

mod preview;
mod widgets;
mod model;
mod render;

pub fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("ui.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: gtk::ApplicationWindow = builder.get_object("window").unwrap();
    window.set_application(application);
    window.set_position(WindowPosition::Center);

    let preview = preview::Preview::new(&builder);
    let bus: Bus = bus::new();
    let widgets = Widgets::new(&builder, &window,  &bus);

    let render_client = RenderClient::new(&bus);
    let model = Model::new(&bus);

    subscribe(&bus, Box::new(widgets.clone()));
    subscribe(&bus, Box::new(render_client));
    subscribe(&bus, Box::new(preview));
    subscribe(&bus, Box::new(model));

    window.show_all();
}
