use gtk;
use gtk::prelude::*;
use gio;
use glib::types::Type as GType;
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
}

impl MgApplication {

    pub fn new(gapp: &gtk::Application) -> Rc<RefCell<MgApplication>> {

        let builder = gtk::Builder::new_from_string(include_str!("mgwindow.ui"));
        let window: gtk::ApplicationWindow = builder.get_object("main_window").unwrap();
        let erase_checkbtn: gtk::CheckButton = builder.get_object("erase_checkbtn").unwrap();
        let model_combo: gtk::ComboBox = builder.get_object("model_combo").unwrap();
        let port_entry: gtk::Entry = builder.get_object("port_entry").unwrap();

        gapp.add_window(&window);

        let app = MgApplication {
            win: window,
            erase_checkbtn: erase_checkbtn,
            model_combo: model_combo,
            port_entry: port_entry,
            device_manager: devices::Manager::new(),
        };
        app.win.connect_delete_event(|_, _| {
            Self::terminate()
        });

        let me = Rc::new(RefCell::new(app));
        {
            let me_too = me.clone();
            me.borrow_mut().model_combo.connect_changed(move |combo| {
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

        me
    }

    fn terminate() -> Inhibit {
        gtk::main_quit();
        Inhibit(false)
    }

    /// Start the app.
    pub fn start(&mut self) {

        self.populate_model_combo();
        self.win.show_all();

        // XXX used the stored value here.
        self.model_changed(&"".to_string());
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
            GType::String, GType::String
                ]);

        utils::setup_text_combo(&self.model_combo, &store);

        let devices = self.device_manager.devices_desc();
        for dev in devices {
            println!("adding dev {:?}", dev);
            utils::add_text_row(&store, dev.id.as_str(), dev.label.as_str());
        }
    }

    fn model_changed(&mut self, id: &String) {
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
        self.device_manager.set_port(id.clone());
    }
}
