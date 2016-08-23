use rustc_serialize::json;
use drivers;
use gpsbabel;

/// Device static capability
#[derive(Clone, Debug, RustcDecodable)]
pub struct Capability {
    pub can_erase: bool,
    pub can_erase_only: bool,
    can_log_enable: bool,
    can_shutoff: bool,
}

/// Describe a device
#[derive(Clone, Debug, RustcDecodable)]
pub struct Desc {
    pub id: String,
    pub label: String,
    cap: Capability,
    driver: String,
}

/// The device database.
#[derive(Clone, Debug, RustcDecodable)]
struct DeviceDb {
    devices: Vec<Desc>,
    drivers: Vec<drivers::Desc>,
}

/// The device manager. Where the magic happens.
pub struct Manager {
    model: Option<String>,
    port: Option<String>,
    devices: Vec<Desc>,
}

impl Manager {
    pub fn new() -> Self {
        let devices_db: DeviceDb = json::decode(
            include_str!("devices.json")
            ).unwrap();
        Manager { model: None, port: None,
                  devices: devices_db.devices }
    }

    pub fn set_model(&mut self, model: String) {
        self.model = Some(model);
    }

    pub fn set_port(&mut self, port: String) {
        self.port = Some(port);
    }

    pub fn devices_desc(&self) -> &Vec<Desc> {
        &self.devices
    }

    pub fn device_capability(&self, model: &String) -> Option<Capability> {
        if model.is_empty() {
            return None;
        }
        // XXX this is suboptimal.
        match self.devices.iter().find(|&device| device.id == *model) {
            Some(device) =>
                Some(device.cap.clone()),
            None =>
                None,
        }
    }

    pub fn get_ports_for_model(&self, _ /*model*/: &String)
                               -> Vec<drivers::Port> {
        return vec![ drivers::Port {
            id: "foo".to_string(), label: "bar".to_string() } ];
    }

    // Get a driver for the device from the current manager.
    pub fn get_device(&self) -> Option<Box<drivers::Driver>> {
        if self.model == None {
            return None;
        }
        let capability: Capability;
        let driver_id = match self.devices.iter().find(
            |&device| {
                if let Some(ref model) = self.model {
                    return &device.id == model;
                }
                false
            }) {
            Some(device) => {
                capability = device.cap.clone();
                device.driver.clone()
            },
            None =>
                return None
        };
        match driver_id.as_str() {
            "m241" |
            "mtk" => {
                match self.port {
                    Some(ref p) =>
                        Some(Box::new(gpsbabel::GpsBabel::new(driver_id,
                                                              p, capability))),
                    _ => None
                }
            },
            _ =>
                None
        }
    }
}

impl Capability {

//    pub fn new() -> Self {
//        Capability {
//            can_erase: false,
//            can_erase_only: false,
//            can_log_enable: false,
//            can_shutoff: false,
//        }
//    }
}


#[cfg(test)]
#[test]
fn test_database() {
    // This test that the database has a valid syntax....
    let devices_db: DeviceDb = json::decode(
        include_str!("devices.json")
            ).unwrap();
    assert!(!devices_db.devices.is_empty());
}
