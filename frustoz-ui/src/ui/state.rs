use std::sync::{Arc, Mutex};
use frustoz_core::model::flame::Flame;
use gdk_pixbuf::Pixbuf;
use gtk::SpinButtonExt;

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
    pub open_file_dialog: gtk::FileChooserNative,
    pub scale_x: gtk::SpinButton,
}

pub enum FlameUpdate {
    ScaleX(f64),
}

pub fn create_state() -> State {
    Arc::new(Mutex::new(init_state()))
}

fn init_state() -> UIState {
    UIState { flame: None, raw: None, components: None, refresh: false }
}

pub fn set_flame(state: &State, flame: Flame) {
    {
        let st = &mut state.lock().unwrap();
        st.refresh = true;
        st.flame = Some(flame);
    }
    crate::render::render(&Arc::clone(&state));
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

pub fn update_flame(state: &State, update: FlameUpdate) {
    {
        let st = &mut state.lock().unwrap();
        if st.refresh { return; }
        st.flame = st.flame.as_ref().map(|mut f| apply_update(f.clone(), update));
    }
    crate::render::render(&Arc::clone(&state));
}

fn apply_update(mut flame: Flame, update: FlameUpdate) -> Flame {
    use FlameUpdate::*;
    match update {
        ScaleX(x) => flame.camera.scale_x = x,
    };
    return flame;
}