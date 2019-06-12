use gdk::prelude::*;
use gtk::{Builder,WindowPosition};
use gtk::prelude::*;

use drawing_area::create;
use frustoz_io as io;
use state::State;

use crate::render;

mod editor;
pub mod state;
mod drawing_area;

pub fn build_ui(application: &gtk::Application) {
    let state: State = state::create_state();
    let glade_src = include_str!("ui.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: gtk::ApplicationWindow = builder.get_object("window").unwrap();
    window.set_application(application);
    window.set_position(WindowPosition::Center);

    let drawing = create(&state);
    let window_box: gtk::Box = builder.get_object("window_box").unwrap();
    window_box.pack_start(&drawing, true, false, 1);

    let scale_x: gtk::SpinButton = builder.get_object("scale_x").unwrap();
    scale_x.connect_value_changed(clone!(state => move |spin_button| {
        let value = spin_button.get_value();
        state::update_flame(&state, state::FlameUpdate::ScaleX(value));
    }));

    let open_file_dialog: gtk::FileChooserNative = gtk::FileChooserNative::new(Some("Open"), Some(&window), gtk::FileChooserAction::Open, None, None);
    open_file_dialog.connect_response(clone!(state => move |dialog, _response| {
        let path = dialog.get_filename();
        let name = path.as_ref().and_then(|x| x.to_str());
        name.map(| name | {
            println!("Trying to read file \"{}\"", name);
            let flame = io::parser::parse_file(name).into_iter().next();
            flame.map(|f| state::set_flame(&state, f));
        });
    dialog.hide();
}));

    let menu_open: gtk::MenuItem = builder.get_object("menu_open").unwrap();
    menu_open.connect_activate(clone!(state => move | _ | {
        let st = &mut state.lock().unwrap();
        println! ("Opening file");
        st.components.as_ref().map( | c | c.open_file_dialog.show());
    }));

    state.lock().unwrap().components = Some(state::Components { drawing, open_file_dialog, scale_x });
    window.show_all();
}

