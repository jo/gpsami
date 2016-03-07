use rustc_serialize::json;


#[derive(Clone, Debug)]
pub struct Port {
    pub id: String,
    pub label: String,
}

/// Device static capability
#[derive(Clone, Debug, RustcDecodable)]
pub struct Capability {
    pub can_erase: bool,
    can_erase_only: bool,
    can_log_enable: bool,
    can_shutoff: bool,
}

#[derive(Clone, Debug, RustcDecodable)]
pub struct Desc {
    pub id: String,
    pub label: String,
    cap: Capability,
    driver: String,
}

#[derive(Clone, Debug, RustcDecodable)]
struct DeviceDb {
    devices: Vec<Desc>,
}

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
        Manager { model: None, port: None, devices: devices_db.devices }
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

    pub fn device_capability(&self, model: &String) -> Capability {
        if model.is_empty() {
            return Capability::new();
        }
        // XXX this is suboptimal.
        match self.devices.iter().find(|&device| device.id == *model) {
            Some(device) =>
                device.cap.clone(),
            None =>
                Capability::new(),
        }
    }

    pub fn get_ports_for_model(&self, _ /*model*/: &String) -> Vec<Port> {
        return vec![ Port { id: "foo".to_string(), label: "bar".to_string() } ];
    }
}

impl Capability {

    pub fn new() -> Self {
        Capability {
            can_erase: false,
            can_erase_only: false,
            can_log_enable: false,
            can_shutoff: false,
        }
    }
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
