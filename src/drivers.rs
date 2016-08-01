use ::Format;


#[derive(Clone, Debug)]
pub struct Port {
    pub id: String,
    pub label: String,
}

#[derive(Clone, Debug, RustcDecodable)]
pub struct Desc {
    id: String,
}

pub trait Driver {
    /// list ports for the device
    fn list_ports(&self) -> Vec<Port>;
    /// open the device
    fn open(&mut self, port: &String) -> bool;
    /// close the device
    fn close(&mut self) -> bool;
    // XXX change types for Result<>
    /// Download the track in specified format
    fn download(&self, format: Format) -> Result<i32, i32>;
    /// Erase the tracks
    fn erase(&self) -> i32;
}

/// GpsBabel "driver". Will use gpsbabel to connect to device.
pub struct GpsBabel {
    device_id: String,
    port: String,
}

impl GpsBabel {
    pub fn new(device: String) -> Self {
        GpsBabel { device_id: device, port: "".to_owned() }
    }
}

impl Driver for GpsBabel {
    fn list_ports(&self) -> Vec<Port> {
        Vec::new()
    }

    fn open(&mut self, port: &String) -> bool {
        self.port = port.to_owned();
        true
    }

    fn close(&mut self) -> bool {
        true
    }

    fn download(&self, _ /*format*/: Format) -> Result<i32, i32>
    {
        Err(0)
    }

    fn erase(&self) -> i32 {
        0
    }

}
