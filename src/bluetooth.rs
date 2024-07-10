use anyhow::{anyhow, Error, Result};
use bluest;
use futures::stream::{self as stream, StreamExt};
use lazy_static::lazy_static;
use simplelog::*;
use uuid;

use crate::status::BatteryStatus;

lazy_static! {
    static ref BATTERY_LEVEL_UUID: uuid::Uuid =
        uuid::Uuid::parse_str("00002a19-0000-1000-8000-00805f9b34fb").unwrap();
    static ref BATTERY_POWER_STATE_UUID: uuid::Uuid =
        uuid::Uuid::parse_str("00002a1a-0000-1000-8000-00805f9b34fb").unwrap();
    static ref BATTERY_LEVEL_STATE_UUID: uuid::Uuid =
        uuid::Uuid::parse_str("00002a1b-0000-1000-8000-00805f9b34fb").unwrap();
}

pub(crate) struct BTBatteryLevel {
    pub name: String,
    pub level: u8,
    pub status: BatteryStatus,
}

impl Default for BTBatteryLevel {
    fn default() -> Self {
        BTBatteryLevel {
            name: "unknown device".into(),
            level: 0,
            status: BatteryStatus::Discharging,
        }
    }
}

fn convert_bytes_to_level(data: Vec<u8>) -> u8 {
    if !data.is_empty() {
        // need only 1st byte
        return data[0];
    }
    0u8
}

#[allow(unused_variables)]
fn convert_bytes_to_status(data: Vec<u8>) -> BatteryStatus {
    // TODO: find a device and implement a proper conversion
    BatteryStatus::Discharging
}

pub(crate) async fn process_device(device: bluest::Device) -> Result<BTBatteryLevel, Error> {
    let name = device.name()?;
    debug!("Device '{}'", &name);
    let mut level = 0u8;
    let mut status = BatteryStatus::Discharging;

    for service in device.services().await? {
        debug!("BT service {:?}", service.uuid());
        for characteristic in service.characteristics().await? {
            if characteristic.uuid() == *BATTERY_LEVEL_UUID {
                level = convert_bytes_to_level(characteristic.value().await?);
                debug!("Battery Level is {:?}", level);
            }
            if characteristic.uuid() == *BATTERY_POWER_STATE_UUID {
                status = convert_bytes_to_status(characteristic.value().await?);
                debug!("Battery Status is {:?}", status);
            }
        }
    }

    Ok(BTBatteryLevel {
        name,
        level,
        status,
    })
}

pub(crate) async fn process_adapter(
    adapter: bluest::Adapter,
) -> Result<Vec<BTBatteryLevel>, Error> {
    debug!("scanning for devices");
    let result = stream::iter(adapter.connected_devices().await?)
        .then(|x| async {
            adapter.connect_device(&x).await?;
            process_device(x).await
        })
        .map(|x| x.unwrap_or_default())
        .collect::<Vec<BTBatteryLevel>>()
        .await;
    Ok(result)
}

pub(crate) async fn get_adapter() -> Result<bluest::Adapter, Error> {
    let adapter = bluest::Adapter::default()
        .await
        .ok_or(anyhow!("Bluetooth adapter not found"))?;
    adapter.wait_available().await?;
    Ok(adapter)
}
