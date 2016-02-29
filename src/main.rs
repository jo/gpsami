extern crate gtk;

use mgapplication::MgApplication;

mod mgapplication;

fn main() {

    let app = MgApplication::new();

    app.start();
}

#[test]
fn it_works() {
}
