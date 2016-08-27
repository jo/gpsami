use std::fmt;
use std::path::PathBuf;

use ::Format;

#[derive(Clone, Debug)]
pub struct Port {
    pub id: String,
    pub label: String,
    pub path: PathBuf,
}

#[derive(Clone, Debug, RustcDecodable)]
pub enum PortType {
    None,
    UsbSerial,
}

#[derive(Clone, Debug, RustcDecodable)]
pub struct Desc {
    pub id: String,
    // the port to look for.
    pub ports: PortType,
}

pub enum Error {
    None,
    Unsupported,
    WrongArg,
    Failed(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::None => write!(f, "{}", "None"),
            Error::Unsupported => write!(f, "{}", "Unsupported"),
            Error::WrongArg => write!(f, "{}", "WrongArg"),
            Error::Failed(ref s) => write!(f, "{}", s)
        }
    }
}

pub trait Driver {
    /// open the device
    fn open(&mut self) -> bool;
    /// close the device
    fn close(&mut self) -> bool;
    /// Download the track in specified format
    /// Return the PathBuf pointing to the datafile.
    fn download(&self, format: Format, erase: bool) -> Result<PathBuf, Error>;
    /// Erase the tracks
    fn erase(&self) -> Error;
}
