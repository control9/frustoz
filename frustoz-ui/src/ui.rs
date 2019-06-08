use core::borrow::BorrowMut;
use std::sync::{Arc, Mutex};

use cairo;
use gdk::prelude::*;
use gdk_pixbuf::Pixbuf;
use gtk::{ApplicationWindow, ApplicationWindowExt, BoxExt, ComboBoxText, SpinButton, WidgetExt, WindowPosition};
use gtk::prelude::*;

use frustoz_core::model::flame::Flame;

use crate::drawing_area::create;
use crate::example;
use crate::example::*;
use crate::render;

pub struct UIState {
    pub flame: Option<Flame>,
    pub raw: Option<Pixbuf>,
    pub components: Option<Components>,
}

pub type State = Arc<Mutex<UIState>>;

pub struct Components {
    pub drawing: gtk::DrawingArea,
    pub example_selector: gtk::ComboBoxText,
}

pub fn build_ui(application: &gtk::Application) {
    let state: State = Arc::new(Mutex::new(init_state()));
    let glade_src = include_str!("ui.glade");
    let builder = gtk::Builder::new_from_string(glade_src);

    let window: gtk::ApplicationWindow = builder.get_object("window").unwrap();
    window.set_application(application);
    window.set_position(WindowPosition::Center);

    let drawing = create(&state);
    let window_box: gtk::Box = builder.get_object("window_box").unwrap();

    window_box.pack_start(&drawing, true, false, 1);

    let example_selector: ComboBoxText = builder.get_object("example_selector").unwrap();
    example_selector.append_text(SPARK_STR);
    example_selector.append_text(SIERPINSKY_STR);
    example_selector.append_text(BARNSLEY_STR);

    example_selector.connect_changed(clone!( state => move |x| {
        on_select(x, &state);
    }));

    let scale_x: SpinButton = builder.get_object("scale_x").unwrap();
    scale_x.connect_activate(clone!(state => move |spin_button| {
        let text = spin_button.get_text().expect("Couldn't get text from spin_button");
        println!("spin_button_input: \"{}\"", text);
        match text.parse::<f64>() {
            Ok(value) => {
                {
                    let st = &mut state.lock().unwrap();
                    st.flame.as_mut().map(|mut f| f.camera.scale_x = value);
                }
                render::render(Arc::clone(&state));
            }
            _ => {}
        };
    }));

    state.lock().unwrap().components = Some(Components { drawing, example_selector });
    window.show_all();
}

fn init_state() -> UIState {
    UIState { flame: None, raw: None, components: None }
}

fn on_select(example_selector: &ComboBoxText, state: &State) {
    select_flame(example_selector, &state);
    render::render(Arc::clone(&state));
}

fn select_flame(example_selector: &ComboBoxText, state: &&Arc<Mutex<UIState>>) {
    let st = &mut state.lock().unwrap();
    let id = example_selector.get_active_text().map(|i| i.to_string());
    st.flame = id.as_ref().map(|x| &**x) // Converting Option<String> to Option<&str> never fails to amuse me
        .and_then(example::get_example);
}