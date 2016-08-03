use std::env;
use std::path::PathBuf;
use std::process::Command;

use ::Format;
use devices::Capability;
use drivers::Error;
use drivers::Driver;
use drivers::Port;

/// GpsBabel "driver". Will use gpsbabel to connect to device.
pub struct GpsBabel {
    device_id: String,
    port: String,
    cap: Capability,
}

impl GpsBabel {
    pub fn new(device: String, capability: Capability) -> Self {
        GpsBabel { device_id: device, port: "".to_owned(), cap: capability }
    }

    /// Return a string associated with the format.
    /// Or None
    fn format_to_string(format: &Format) -> Option<&'static str> {
        match *format {
            Format::Gpx => Some("gpx"),
            Format::Kml => Some("kml"),
            _ => None
        }
    }

    /// Return an extension (with .) associated with the format.
    /// Or None
    fn format_to_extension(format: &Format) -> Option<&'static str> {
        match *format {
            Format::Gpx => Some(".gpx"),
            Format::Kml => Some(".kml"),
            _ => None
        }
    }

    /// Build the basic command line for the device on port, eventually for delete
    /// after download or erase only.
    fn build_basic_command_line(device_id: &str, port: &str,
                                erase: bool, erase_only: bool) -> Command {
        let mut device_string = String::from(device_id);
        if erase {
            device_string.push_str(",erase");
        }
        if erase_only {
            device_string.push_str(",erase_only");
        }
        let mut command = Command::new("gpsbabel");
        command.arg("-t")
            .arg("-w")
            .arg("-i").arg(device_string)
            .arg("-f").arg(port);

        return command
    }
}

impl Driver for GpsBabel {
    fn list_ports(&self) -> Vec<Port> {
        // XXX implement
        Vec::new()
    }

    fn open(&mut self, port: &String) -> bool {
        self.port = port.to_owned();
        true
    }

    fn close(&mut self) -> bool {
        true
    }

    /// Download the data into a file. Return the PathBuf to said file on success.
    /// Caller is responsible for deleting the file.
    fn download(&self, format: Format, erase: bool) -> Result<PathBuf, Error>
    {
        // we requested erase at the same time and it is not supported.
        if erase && !self.cap.can_erase {
            return Err(Error::Unsupported)
        }

        let fmt_string_opt = Self::format_to_string(&format);
        if fmt_string_opt.is_none() {
            // invalid format
            return Err(Error::WrongArg);
        }
        let fmt_string = fmt_string_opt.unwrap();

        let extension_opt = Self::format_to_extension(&format);
        if extension_opt.is_none() {
            // invalid format
            return Err(Error::WrongArg);
        }
        let extension = extension_opt.unwrap();

        // XXX use a better temporary name
        let mut dir = env::temp_dir();
        dir.push(String::from("magellan.") + extension);

        /* gpsbabel -t -w -i m241 -f /dev/ttyACM0 -o gpx -F $1 */
        let output = GpsBabel::build_basic_command_line(&self.device_id, &self.port, erase, false)
            .arg("-o").arg(fmt_string) // format
            .arg("-F").arg(String::from(dir.to_str().unwrap()))
            .output()
            .expect("failed to execute process");
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        if output.status.success() {
            Ok(dir)
        } else {
            Err(Error::Failed)
        }
    }

    /// Erase the logs on the device. Return an error if not capable.
    fn erase(&self) -> Error {
        // Device doesn't support "erase only"
        if !self.cap.can_erase_only {
            return Error::Unsupported
        }
        /* gpsbabel -t -w -i m241,erase_only -f /dev/ttyACM0 */
        let output = GpsBabel::build_basic_command_line(&self.device_id, &self.port, false, true)
            .output()
            .expect("failed to execute process");
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        if output.status.success() {
            Error::None
        } else {
            Error::Failed
        }
    }

}

#[test]
fn test_command_builder() {
    let command = GpsBabel::build_basic_command_line("foo", "ttyS0", false, false);
    assert_eq!(format!("{:?}", command),
               "\"gpsbabel\" \"-t\" \"-w\" \"-i\" \"foo\" \"-f\" \"ttyS0\"")
}

#[test]
fn test_format() {
    let result = GpsBabel::format_to_string(&Format::Gpx);
    assert!(result.is_some());
    assert_eq!(result, Some("gpx"));

    let result = GpsBabel::format_to_string(&Format::None);
    assert!(result.is_none());
}

#[test]
fn test_extensions() {
    let result = GpsBabel::format_to_extension(&Format::Gpx);
    assert!(result.is_some());
    assert_eq!(result, Some(".gpx"));

    let result = GpsBabel::format_to_string(&Format::None);
    assert!(result.is_none());
}
