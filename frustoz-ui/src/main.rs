extern crate frustoz_core;
extern crate gdk;
extern crate gio;
extern crate gdk_pixbuf;
extern crate glib;
extern crate gtk;
#[macro_use]
extern crate log;
extern crate cairo;

extern crate num_cpus;
extern crate rayon;

use std::fs::File;
use std::env;

use rayon::ThreadPoolBuilder;

use gdk::prelude::*;
use gtk::prelude::*;
use gio::prelude::*;
use gtk::prelude::GtkApplicationExt;
use gdk_pixbuf::{Colorspace, Pixbuf};
use simplelog::*;

use frustoz_core::model::flame::Flame;
use frustoz_core::render;

mod ui;

pub const PRESERVE_CPUS: u32 = 1;

fn main() {
    let threads = (num_cpus::get() as u32 - PRESERVE_CPUS).max(1);
    ThreadPoolBuilder::new().num_threads(threads as usize).build_global().expect("Failed to initialize pool");

    let application = gtk::Application::new("com.github.gtk-rs.examples.basic",
                                            Default::default())
        .expect("Initialization failed...");
    application.connect_activate(|app| {
        ui::build_ui(app);
    });
    application.run(&env::args().collect::<Vec<_>>());
}