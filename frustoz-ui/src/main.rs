extern crate frustoz_core;
extern crate frustoz_io;
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
use std::sync::{Arc, Mutex};


pub const PRESERVE_CPUS: u32 = 1;

macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

mod ui;
mod drawing_area;
mod render;
mod example;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default()).unwrap(),
        ]
    ).unwrap();

    let threads = (num_cpus::get() as u32 - PRESERVE_CPUS).max(1);
    ThreadPoolBuilder::new().num_threads(threads as usize).build_global().expect("Failed to initialize pool");

    let application = gtk::Application::new("name.control9.frustoz",
                                            Default::default())
        .expect("Initialization failed...");
    application.connect_activate(|app| {
        ui::build_ui(app);
    });
    application.run(&env::args().collect::<Vec<_>>());
}