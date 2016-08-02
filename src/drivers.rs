use std::path::PathBuf;

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

pub enum Error {
    None,
    Unsupported,
    WrongArg,
    Failed
}

pub trait Driver {
    /// list ports for the device
    fn list_ports(&self) -> Vec<Port>;
    /// open the device
    fn open(&mut self, port: &String) -> bool;
    /// close the device
    fn close(&mut self) -> bool;
    /// Download the track in specified format
    /// Return the PathBuf pointing to the datafile.
    fn download(&self, format: Format, erase: bool) -> Result<PathBuf, Error>;
    /// Erase the tracks
    fn erase(&self) -> Error;
}

