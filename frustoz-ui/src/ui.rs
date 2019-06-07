use core::borrow::BorrowMut;
use std::sync::{Arc, Mutex};

use cairo;
use gdk::prelude::*;
use gdk_pixbuf::Pixbuf;
use gtk::{WidgetExt, WindowPosition, ComboBoxText};
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
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_position(WindowPosition::Center);
    window.set_default_size(350, 70);

    let bx = gtk::FlowBox::new();
    window.add(&bx);

    let drawing = create(&state);

    let example_selector = gtk::ComboBoxText::new();
    example_selector.append_text(SPARK_STR);
    example_selector.append_text(SIERPINSKY_STR);
    example_selector.append_text(BARNSLEY_STR);

    example_selector.connect_changed(clone!( state => move |x| {
        on_select(x, &state);
    }));
    bx.add(&example_selector);
    bx.add(&drawing);

    state.lock().unwrap().components = Some(Components{drawing, example_selector});
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