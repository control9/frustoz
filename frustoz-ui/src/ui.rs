use core::borrow::BorrowMut;
use std::sync::{Arc, Mutex};

use cairo;
use gdk::prelude::*;
use gdk_pixbuf::Pixbuf;
use gtk::{ApplicationWindow, ApplicationWindowExt, BoxExt, Builder, ComboBoxText, DialogExt, FileChooserExt, GtkMenuItemExt, SpinButton, WidgetExt, WindowPosition};
use gtk::prelude::*;

use frustoz_core::model::flame::Flame;
use frustoz_io as io;

use crate::drawing_area::create;
use crate::example;
use crate::example::*;
use crate::render;

pub struct UIState {
    pub flame: Option<Flame>,
    pub raw: Option<Pixbuf>,
    pub components: Option<Components>,
    pub refresh: bool,
}

pub type State = Arc<Mutex<UIState>>;

#[derive(Clone)]
pub struct Components {
    pub drawing: gtk::DrawingArea,
    pub example_selector: gtk::ComboBoxText,
    pub open_file_dialog: gtk::FileChooserNative,
    pub scale_x: SpinButton,
}

enum FlameUpdate {
    ScaleX(f64),
}

pub fn build_ui(application: &gtk::Application) {
    let state: State = Arc::new(Mutex::new(init_state()));
    let glade_src = include_str!("ui.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: gtk::ApplicationWindow = builder.get_object("window").unwrap();
    window.set_application(application);
    window.set_position(WindowPosition::Center);

    let drawing = create(&state);
    let window_box: gtk::Box = builder.get_object("window_box").unwrap();

    window_box.pack_start(&drawing, true, false, 1);

    let example_selector = init_example_selector(&builder, &state);

    let scale_x: SpinButton = builder.get_object("scale_x").unwrap();
    scale_x.connect_value_changed(clone!(state => move |spin_button| {
        let value = spin_button.get_value();
        println!("spin_button_input: \"{}\"", value);
        update_flame(&state, FlameUpdate::ScaleX(value));
    }));

    let open_file_dialog: gtk::FileChooserNative = gtk::FileChooserNative::new(Some("Open"), Some(&window), gtk::FileChooserAction::Save, None, None);
    open_file_dialog.connect_response(clone!(state =>move |dialog, _response| {
        let path = dialog.get_filename().unwrap();
        let name = path.to_str().unwrap();
        println!("Trying to read file \"{}\"", name);
        let flame = io::parser::parse_file(name).into_iter().next();
        flame.map(|f| set_flame(&state, f));
        dialog.hide();
    }));

    let menu_open: gtk::MenuItem = builder.get_object("menu_open").unwrap();
    menu_open.connect_activate(clone!(state => move |_| {
        let st : &mut UIState = &mut state.lock().unwrap();
        println!("Opening file");
        st.components.as_ref().map(|c| c.open_file_dialog.show());
    }));

    state.lock().unwrap().components = Some(Components { drawing, example_selector, open_file_dialog, scale_x });
    window.show_all();
}

fn init_example_selector(builder: &Builder, state: &State) -> ComboBoxText {
    let example_selector: ComboBoxText = builder.get_object("example_selector").unwrap();
    example_selector.append_text(SPARK_STR);
    example_selector.append_text(SIERPINSKY_STR);
    example_selector.append_text(BARNSLEY_STR);
    example_selector.connect_changed(clone!( state => move |x| {
        on_select(x, &state);
    }));
    example_selector
}

fn init_state() -> UIState {
    UIState { flame: None, raw: None, components: None, refresh: false }
}

fn on_select(example_selector: &ComboBoxText, state: &State) {
    let flame = select_flame(example_selector);
    flame.map(|f| set_flame(state, f));
}

fn select_flame(example_selector: &ComboBoxText) -> Option<Flame> {
    let id = example_selector.get_active_text().map(|i| i.to_string());
    id.as_ref().map(|x| &**x) // Converting Option<String> to Option<&str> never fails to amuse me
        .and_then(example::get_example)
}

fn set_flame(state: &State, flame: Flame) {
    {
        let st = &mut state.lock().unwrap();
        st.refresh = true;
        st.flame = Some(flame);
    }
    render::render(&Arc::clone(&state));
    rebind_state(&Arc::clone(&state));
    {
        let st = &mut state.lock().unwrap();
        st.refresh = false;
    }
}

fn rebind_state(state: &State) {
    let (flame, components) = {
        let st = &mut state.lock().unwrap();
        (st.flame.as_ref().unwrap().clone(), st.components.as_ref().unwrap().clone())
    };
    components.scale_x.set_value(flame.camera.scale_x);
}

fn update_flame(state: &State, update: FlameUpdate) {
    {
        let st = &mut state.lock().unwrap();
        if st.refresh { return; }
        st.flame = st.flame.as_ref().map(|mut f| apply_update(f.clone(), update));
    }
    render::render(state);
}

fn apply_update(mut flame: Flame, update: FlameUpdate) -> Flame {
    use FlameUpdate::*;
    match update {
        ScaleX(x) => flame.camera.scale_x = x,
    };
    return flame;
}