use gtk;
use gtk::traits::*;
use gtk::signal::Inhibit;
use glib::types::Type as GType;
use std::rc::Rc;
use std::cell::RefCell;

use devices;
use utils;

pub struct MgApplication {
    win: gtk::Window,
    download_btn: gtk::Button,
    erase_checkbtn: gtk::CheckButton,
    model_combo: gtk::ComboBox,
    port_combo: gtk::ComboBox,
    port_combo_store: gtk::ListStore,

    device_manager: devices::Manager,
}

impl MgApplication {

    /// Init Gtk and stuff. Called by the contructor.
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

    pub fn new() -> Rc<RefCell<MgApplication>> {
        Self::init();

        let builder: gtk::widgets::Builder =
            gtk::widgets::Builder::new_from_string(include_str!("mgwindow.ui")).
            unwrap();
        let window: gtk::Window =
            unsafe { builder.get_object("main_window") }.unwrap();
        let download_btn: gtk::Button =
            unsafe { builder.get_object("download_btn") }.unwrap();
        download_btn.set_sensitive(false);
        let erase_checkbtn: gtk::CheckButton =
            unsafe { builder.get_object("erase_checkbtn") }.unwrap();
        let model_combo: gtk::ComboBox =
            unsafe { builder.get_object("model_combo") }.unwrap();
        let port_combo: gtk::ComboBox =
            unsafe { builder.get_object("port_combo") }.unwrap();

        let store = gtk::ListStore::new(&[
            GType::String, GType::String
                ]).unwrap();

        let app = MgApplication {
            win: window,
            download_btn: download_btn,
            erase_checkbtn: erase_checkbtn,
            model_combo: model_combo,
            port_combo: port_combo,
            port_combo_store: store,
            device_manager: devices::Manager::new(),
        };
        app.win.connect_delete_event(|_, _| {
            Self::terminate()
        });
        app.download_btn.connect_clicked(|_| {
            //
        });

        let me = Rc::new(RefCell::new(app));
        let me2 = me.clone();
        me.borrow_mut().model_combo.connect_changed(move |combo| {
            if let Some(id) = combo.get_active_id() {
                me2.borrow_mut().model_changed(&id);
            }
        });

        let me3 = me.clone();
        me.borrow_mut().port_combo.connect_changed(move |combo| {
            if let Some(id) = combo.get_active_id() {
                me3.borrow_mut().port_changed(&id);
            }
        });

        me
    }

    fn terminate() -> Inhibit {
        gtk::main_quit();
        Inhibit(false)
    }

    /// Start the app.
    pub fn start(&mut self) {
        self.populate_model_combo();
        self.setup_port_combo();
        self.win.show_all();

        // XXX used the stored value here.
        self.model_changed(&"".to_string());
    }

    fn setup_port_combo(&mut self) {
        let model = self.port_combo_store.get_model().unwrap();

        utils::setup_text_combo(&self.port_combo, model);
    }

    fn populate_port_combo(&mut self, ports: &Vec<devices::Port>) {
        self.port_combo_store.clear();
        for port in ports {
            println!("adding port {:?}", port);
            utils::add_text_row(&self.port_combo_store,
                                port.id.as_str(),
                                port.label.as_str());
        }
    }

    fn populate_model_combo(&mut self) {
        let store = gtk::ListStore::new(&[
            GType::String, GType::String
                ]).unwrap();

        let model = store.get_model().unwrap();

        utils::setup_text_combo(&self.model_combo, model);

        let devices = self.device_manager.devices_desc();
        for dev in devices {
            println!("adding dev {:?}", dev);
            utils::add_text_row(&store, dev.id.as_str(), dev.label.as_str());
        }
    }

    fn model_changed(&mut self, id: &String) {
        let cap = self.device_manager.device_capability(id);
        self.update_device_capability(&cap);
        self.device_manager.set_model(id.clone());
        let ports = self.device_manager.get_ports_for_model(id);
        self.populate_port_combo(&ports);
    }

    fn update_device_capability(&self, capability: &devices::Capability) {
        self.erase_checkbtn.set_sensitive(capability.can_erase);
    }

    fn port_changed(&mut self, id: &String) {
        self.device_manager.set_port(id.clone());
    }
}
