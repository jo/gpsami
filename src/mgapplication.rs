use gtk::widgets;
use gtk;
use gtk::traits::*;
use gtk::signal::Inhibit;

pub struct MgApplication {
    win: widgets::Window,
}

impl MgApplication {

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

    pub fn new() -> MgApplication {
        Self::init();

        let window: widgets::Window;
        if let Some(b) = widgets::Builder::new_from_string(
            include_str!("mgwindow.ui")) {
            window =
                unsafe { b.get_object("main_window") }.unwrap();
            window.show_all();
        }
        else {
            window = widgets::Window::new(gtk::WindowType::Toplevel).unwrap();
        }

        MgApplication {
            win: window
        }
    }

    fn terminate() -> Inhibit {
        gtk::main_quit();
        Inhibit(false)
    }

    pub fn start(&self) {
        self.win.connect_delete_event(|_, _| {
            Self::terminate()
        });

        gtk::main();
    }
}
