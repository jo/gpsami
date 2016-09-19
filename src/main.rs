extern crate gio;
extern crate glib;
extern crate gtk;
extern crate libudev;
extern crate gudev;
extern crate rustc_serialize;

use gtk::prelude::*;

use mgapplication::MgApplication;

mod mgapplication;
mod devices;
mod drivers;
mod gpsbabel;
mod utils;


pub enum Format {
    None,
    Gpx,
    Kml,
}

/// Init Gtk and stuff.
fn init() {
    use std::sync::{Once, ONCE_INIT};

    static START: Once = ONCE_INIT;

    START.call_once(|| {
        // run initialization here
        if gtk::init().is_err() {
            panic!("Failed to initialize GTK.");
        }
    });
}

fn main() {

    init();

    let gapp = gtk::Application::new(Some("net.figuiere.Magellan"),
                                         gio::APPLICATION_FLAGS_NONE).unwrap();

    gapp.connect_activate(move |gapp| {
        let app = MgApplication::new(&gapp);

        app.borrow_mut().start();
    });

    gapp.run(0, &[]);
}

#[test]
fn it_works() {
}
