use gtk::{Builder, SpinButton, SpinButtonExt, SpinButtonSignals};
use glib::Continue;

use crate::ui::bus::Bus;
use crate::ui::bus::process;
use crate::ui::bus::FlameUpdate::*;
use crate::ui::bus::Event::Edit;
use frustoz_core::model::flame::Flame;


#[derive(Clone)]
pub struct CameraWidgets {
    pub scale_x: SpinButton,
    scale_y: SpinButton,
    origin_x: SpinButton,
    origin_y: SpinButton,
}

impl CameraWidgets {
    pub fn new(builder: &Builder, bus: &Bus) -> Self {
        let scale_x: SpinButton = builder.get_object("scale_x").unwrap();
        scale_x.connect_value_changed(clone!(bus => move |spin_button| {
            let value = spin_button.get_value();
            process(&bus, Edit(ScaleX(value)));
        }));

        let scale_y: SpinButton = builder.get_object("scale_y").unwrap();
        scale_y.connect_value_changed(clone!(bus => move |spin_button| {
            let value = spin_button.get_value();
            process(&bus, Edit(ScaleY(value)));
        }));

        let origin_x: SpinButton = builder.get_object("origin_x").unwrap();
        origin_x.connect_value_changed(clone!(bus => move |spin_button| {
            let value = spin_button.get_value();
            process(&bus, Edit(OriginX(value)));
        }));

        let origin_y: SpinButton = builder.get_object("origin_y").unwrap();
        origin_y.connect_value_changed(clone!(bus => move |spin_button| {
            let value = spin_button.get_value();
            process(&bus, Edit(OriginY(value)));
        }));

        CameraWidgets {
            scale_x, scale_y, origin_x, origin_y
        }
    }

    pub fn bind(&self, flame: &Flame) {
        let itself = self.clone();

        gtk::idle_add(clone!(flame => move || {
            itself.scale_x.set_value(flame.camera.scale_x);
            itself.scale_y.set_value(flame.camera.scale_y);
            itself.origin_x.set_value(flame.camera.origin.0);
            itself.origin_y.set_value(flame.camera.origin.1);
            Continue(false)
        }));
    }
}
