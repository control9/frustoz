extern crate gtk;
extern crate gdk_pixbuf;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

extern crate glib;
extern crate frustoz_core;


use frustoz_core::render;
use glib::Bytes;

use gtk::{
    Cast,
    ContainerExt,
    EditableSignals,
    Entry,
    EntryExt,
    Inhibit,
    Label,
    LabelExt,
    WidgetExt,
    Window,
    WindowType,
    FlowBoxExt,
    OrientableExt,
    Orientable,
    Orientation,
    Box,
    Image,
};

use gtk::Orientation::Vertical;
use gtk::{GtkWindowExt, ButtonExt};
use relm::{Relm, Update, Widget, WidgetTest};

use self::Msg::*;
use gdk_pixbuf::{Pixbuf, Colorspace};

struct Model {
    content: String,
}

#[derive(Msg)]
enum Msg {
    Change,
    Quit,
    Reset,
}

struct Win {
    model: Model,
    widgets: Widgets,
}

struct Widgets {
    input: Entry,
    label: Label,
    window: Window,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
            content: String::new(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Change => {
                self.model.content = self.widgets.input.get_text()
                    .expect("get_text failed")
                    .chars()
                    .rev()
                    .collect();
                self.widgets.label.set_text(&self.model.content);
            },
            Reset => {
                self.model.content = "".to_owned();
                self.widgets.label.set_text(&self.model.content);
                self.widgets.input.set_text(&self.model.content);
            }
            Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.widgets.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let vbox = gtk::FlowBox::new();
        let vbox_or = vbox.clone().upcast::<gtk::Orientable>();
        vbox_or.set_orientation(Vertical);

        let input = Entry::new();
        vbox.add(&input);

        let label = Label::new(None);
        label.set_max_width_chars(12);

        vbox.add(&label);

        let button = gtk::Button::new_with_label("Click me!");

        vbox.add(&button);

        let flame = frustoz_core::example::spark();
        let raw = frustoz_core::render::simple_renderer::render::<render::NoOpReporter>(flame);
        let raw_bytes = Bytes::from(&raw);
        let buf = Pixbuf::new_from_bytes(&raw_bytes,Colorspace::Rgb, false,8,1024, 768, 3 * 1024);
        let pic = Image::new_from_pixbuf(Some(&buf));

        vbox.add(&pic);

        let window = Window::new(WindowType::Toplevel);

        window.add(&vbox);
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(1200, 800);
        window.show_all();

        connect!(relm, input, connect_changed(_), Change);
        connect!(relm, button, connect_clicked(_), Reset);
        connect!(relm, window, connect_delete_event(_, _), return (Some(Quit), Inhibit(false)));

        Win {
            model,
            widgets: Widgets {
                input,
                label,
                window,
            },
        }
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}