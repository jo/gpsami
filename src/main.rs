extern crate gtk;
extern crate glib;

use mgapplication::MgApplication;

mod mgapplication;
mod devices;
mod utils;

fn main() {

    let app = MgApplication::new();

    app.borrow_mut().start();

    gtk::main();
}

#[test]
fn it_works() {
}
