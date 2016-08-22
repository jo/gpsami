use gio;
use glib;
use gtk;
use gtk::prelude::*;

use std;
use std::path;
use std::rc::Rc;
use std::cell::RefCell;

use devices;
use drivers;
use utils;
use ::Format;

pub struct MgApplication {
    win: gtk::ApplicationWindow,
    erase_checkbtn: gtk::CheckButton,
    model_combo: gtk::ComboBox,
    port_entry: gtk::Entry,

    device_manager: devices::Manager,
    prefs_store: glib::KeyFile,

    model_changed_signal: u64,
}

impl MgApplication {

    pub fn new(gapp: &gtk::Application) -> Rc<RefCell<MgApplication>> {

        let builder = gtk::Builder::new_from_string(include_str!("mgwindow.ui"));
        let window: gtk::ApplicationWindow = builder.get_object("main_window").unwrap();
        let erase_checkbtn: gtk::CheckButton = builder.get_object("erase_checkbtn").unwrap();
        let model_combo: gtk::ComboBox = builder.get_object("model_combo").unwrap();
        let port_entry: gtk::Entry = builder.get_object("port_entry").unwrap();

        gapp.add_window(&window);

        let mut app = MgApplication {
            win: window,
            erase_checkbtn: erase_checkbtn,
            model_combo: model_combo,
            port_entry: port_entry,
            device_manager: devices::Manager::new(),
            prefs_store: glib::KeyFile::new(),
            model_changed_signal: 0,
        };

        let me = Rc::new(RefCell::new(app));
        {
            let me_too = me.clone();
            app.model_changed_signal = me.borrow_mut().model_combo.connect_changed(move |combo| {
                if let Some(id) = combo.get_active_id() {
                    me_too.borrow_mut().model_changed(&id);
                }
            });
        }
        {
            let me_too = me.clone();
            me.borrow_mut().port_entry.connect_changed(move |entry| {
                if let Some(id) = entry.get_text() {
                    me_too.borrow_mut().port_changed(&id);
                }
            });
        }
        {
            let me_too = me.clone();
            let dload_action = gio::SimpleAction::new("download", None);
            dload_action.connect_activate(move |_,_| {
                let driver = me_too.borrow().device_manager.get_driver();
                if driver.is_none() {
                    println!("nodriver");
                } else {
                    let mut d = driver.unwrap();
                    d.open(&me_too.borrow().device_manager.get_port());
                    let output = d.download(Format::Gpx, false);
                    if output.is_ok() {
                        println!("success {}", output.ok().unwrap().to_str().unwrap());
                    } else {

                        match output.err() {
                            Some(e) => println!("error {}", e),
                            _ => println!("error unknown")
                        }
                    }
                }
            });
            me.borrow_mut().win.add_action(&dload_action);
        }

        {
            let erase_action = gio::SimpleAction::new("erase", None);
            erase_action.connect_activate(move |_,_| {

            });
            me.borrow_mut().win.add_action(&erase_action);
        }

        me.borrow_mut().load_settings();
        me
    }

    fn settings_dir() -> path::PathBuf {
        // XXX replace this by glib stuff when we can.
        // Also we treat a failure of this as fatal.
        let mut path: path::PathBuf = std::env::home_dir().unwrap();
        path.push(".magellan");
        path
    }

    fn save_settings(&self) -> Result<(), glib::Error> {
        let mut path = Self::settings_dir();
        path.push("magellan.ini");
        self.prefs_store.save_to_file(path.to_str().unwrap())
    }

    pub fn load_settings(&mut self) -> Result<(), glib::Error> {
        let mut path = Self::settings_dir();
        match std::fs::create_dir_all(path.clone()) {
            Err(e) =>
                return Err(
                    glib::Error::new(glib::FileError::Failed,
                                     &format!("Can't create settings dir '{:?}': {}", path, e))),
            Ok(_) => {
            },
        }
        path.push("magellan.ini");

        match self.prefs_store.load_from_file(path, glib::KEY_FILE_NONE) {
            Err(e) => {
                println!("error with g_key_file {}", e);
                Err(e)
            },
            Ok(_) => {
                Ok(())
            },
        }
    }

    /// Start the app.
    pub fn start(&mut self) {

        self.populate_model_combo();
        self.win.show_all();

        // XXX used the stored value here.
    }

    fn populate_port_combo(&mut self, /*ports*/ _: &Vec<drivers::Port>) {
// XXX fix
//        self.port_combo.remove_all();
//        for port in ports {
//            println!("adding port {:?}", port);
//            self.port_combo.append_text(port.id.as_str());
//        }
    }

    fn populate_model_combo(&mut self) {
        let store = gtk::ListStore::new(&[
            gtk::Type::String, gtk::Type::String
                ]);

        utils::setup_text_combo(&self.model_combo, &store);

        {
            let devices = self.device_manager.devices_desc();
            for dev in devices {
                println!("adding dev {:?}", dev);
                utils::add_text_row(&store, dev.id.as_str(), dev.label.as_str());
            }
        }

        let model = self.prefs_store.get_string("device", "model").unwrap_or("".to_string());

        // XXX this is a hack to not have the signal called as we'll end up
        // recursively borrow_mut self via the RefCell
        glib::signal_handler_block(&self.model_combo, self.model_changed_signal);
        self.model_combo.set_active_id(Some(&model));
        self.model_changed(&model);
        glib::signal_handler_block(&self.model_combo, self.model_changed_signal);
    }

    fn model_changed(&mut self, id: &String) {
        println!("model changed to {}", id);
        self.prefs_store.set_string("device", "model", &id);
        self.save_settings();

        let cap = self.device_manager.device_capability(id);
        if cap.is_some() {
            self.update_device_capability(&cap.unwrap());
            self.device_manager.set_model(id.clone());
            let ports = self.device_manager.get_ports_for_model(id);
            self.populate_port_combo(&ports);
        } else {
            // XXX clear device.
        }
    }

    fn update_device_capability(&self, capability: &devices::Capability) {
        self.erase_checkbtn.set_sensitive(capability.can_erase);
    }

    fn port_changed(&mut self, id: &String) {
        self.prefs_store.set_string("device", "port", &id);
        self.save_settings();

        self.device_manager.set_port(id.clone());
    }
}
