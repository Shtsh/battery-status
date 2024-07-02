use anyhow::{anyhow, bail, Error, Result};
use bluest;
use futures::stream::{self as stream, StreamExt};
use lazy_static::lazy_static;
use simplelog::*;
use uuid;

lazy_static! {
    static ref BATTERY_UUID: uuid::Uuid =
        uuid::Uuid::parse_str("00002a19-0000-1000-8000-00805f9b34fb").unwrap();
}

pub struct BTBatteryLevel {
    pub name: String,
    pub level: u8,
}

impl Default for BTBatteryLevel {
    fn default() -> Self {
        BTBatteryLevel {
            name: "unknown device".into(),
            level: 0,
        }
    }
}

pub async fn process_device(device: bluest::Device) -> Result<BTBatteryLevel, Error> {
    let name = device.name()?;
    debug!("Device '{}'", &name);
    let result = stream::iter(device.services().await?)
        .then(|service| async move { service.characteristics().await.unwrap() })
        .map(|x| stream::iter(x))
        .flatten()
        .filter_map(|x| async move {
            if x.uuid() == *BATTERY_UUID {
                let level = x.value().await.unwrap()[0];
                debug!("Battery Level is {:?}", level);
                Some(level)
            } else {
                None
            }
        })
        .collect::<Vec<u8>>()
        .await;

    if result.is_empty() {
        bail!("Unable to detect battery level");
    }
    let level = result[0];
    Ok(BTBatteryLevel { name, level })
}

pub async fn process_adapter(adapter: bluest::Adapter) -> Result<Vec<BTBatteryLevel>, Error> {
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

pub async fn get_adapter() -> Result<bluest::Adapter, Error> {
    let adapter = bluest::Adapter::default()
        .await
        .ok_or(anyhow!("Bluetooth adapter not found"))?;
    adapter.wait_available().await?;
    Ok(adapter)
}
