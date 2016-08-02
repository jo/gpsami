extern crate gtk;
extern crate glib;
extern crate rustc_serialize;

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

fn main() {

    let app = MgApplication::new();

    app.borrow_mut().start();

    gtk::main();
}

#[test]
fn it_works() {
}
