use std::{str::FromStr, time::Duration};

use anyhow::{anyhow, bail, Error, Result};
use lazy_static::lazy_static;
use serialport;
use simplelog::*;

use crate::status::BatteryStatus;
const VENDOR_ID: u16 = 13807; // Dygma

lazy_static! {
    static ref SUPPORTED_DEVICES: Vec<u16> = vec![
        18, // Defy
    ];
}

impl FromStr for BatteryStatus {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "4" => Ok(BatteryStatus::Unknown),
            "3" => Ok(BatteryStatus::Unknown),
            "2" => Ok(BatteryStatus::Charged),
            "1" => Ok(BatteryStatus::Charging),
            "0" => Ok(BatteryStatus::Discharging),
            _ => Err(anyhow!(
                "Error converting BatteryStatus: unknown value {}",
                &s
            )),
        }
    }
}

#[allow(dead_code)]
pub(crate) struct DygmaBatteryInfo {
    pub name: String,
    pub left_level: u8,
    pub left_status: BatteryStatus,
    pub right_level: u8,
    pub right_status: BatteryStatus,
}

impl DygmaBatteryInfo {
    pub fn from_port(port: Box<dyn serialport::SerialPort>) -> Result<Self, Error> {
        let mut sender = DygmaSerialPort::new(port);
        let left_level = sender
            .send_command("wireless.battery.left.level")?
            .parse::<u8>()?;
        let left_status =
            BatteryStatus::from_str(&sender.send_command("wireless.battery.left.status")?)?;
        let right_level = sender
            .send_command("wireless.battery.right.level")?
            .parse::<u8>()?;
        let right_status =
            BatteryStatus::from_str(&sender.send_command("wireless.battery.right.status")?)?;

        Ok(DygmaBatteryInfo {
            name: "Dygma Defy".into(),
            left_level,
            left_status,
            right_level,
            right_status,
        })
    }
}

struct DygmaSerialPort {
    path: String,
    inner: Box<dyn serialport::SerialPort>,
}

impl DygmaSerialPort {
    fn new(inner: Box<dyn serialport::SerialPort>) -> Self {
        let path = inner.name().unwrap_or_default();
        Self { inner, path }
    }

    fn send_command(&mut self, command: &str) -> Result<String, Error> {
        debug!("Sending {command} to {}", self.path);

        let written = self.inner.write(format!("{command}\n").as_bytes())?;
        debug!("written {written} bytes to {}", self.path);

        let mut serial_buf: Vec<u8> = vec![0; 50];
        self.inner.read(serial_buf.as_mut_slice())?;
        let response = String::from_utf8(serial_buf)?;
        debug!("Response from {}: {}", self.path, &response);
        // we need only one line here
        let parts: Vec<&str> = response.split("\r\n").collect();
        if parts.len() < 2 {
            bail!("Got empty response from {}", self.path);
        }
        let result = parts[0].trim().into();
        debug!("execution result is \"{}\"", result);
        Ok(result)
    }
}

pub fn get_battery_info(path: String) -> Result<DygmaBatteryInfo, Error> {
    debug!("Opening {}", &path);
    let port = serialport::new(&path, 115_200)
        .timeout(Duration::from_millis(100))
        .open()?;

    DygmaBatteryInfo::from_port(port)
}

pub fn list_devices() -> Result<Vec<serialport::SerialPortInfo>, Error> {
    debug!("trying to detect dygma neuron");
    let mut discovered: Vec<serialport::SerialPortInfo> = vec![];
    for port in serialport::available_ports()? {
        if !port.port_name.starts_with("/dev/tty") {
            continue;
        }
        match &port.port_type {
            serialport::SerialPortType::UsbPort(usbinfo) => {
                if !(usbinfo.vid == VENDOR_ID) {
                    continue;
                };
                if !SUPPORTED_DEVICES.contains(&usbinfo.pid) {
                    continue;
                };
                discovered.push(port.clone());
            }
            _ => continue,
        }
    }
    Ok(discovered)
}
