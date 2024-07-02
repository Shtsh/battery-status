use anyhow::{bail, Error, Result};
use std::env;

#[cfg(target_os = "macos")]
use objc2_core_bluetooth as cb;

#[cfg(target_os = "macos")]
fn has_bluetooth_permission() -> Result<bool, Error> {
    unsafe {
        let manager = cb::CBCentralManager::new();
        return Ok(manager.authorization() == cb::CBManagerAuthorization::AllowedAlways);
    }
}

#[cfg(not(target_os = "macos"))]
fn has_bluetooth_permission() -> Result<bool, Error> {
    return Ok(true);
}

pub fn check_permissions() -> Result<(), Error> {
    if !has_bluetooth_permission()? {
        if env::consts::OS == "macos" {
            bail!("Unable to get adapeters. Probably blutooth access is not allowed. Please enable it in System Preferences → Security & Privacy → Privacy → Bluetooth");
        }
        bail!("Unable to get bluetooth permissions");
    }
    Ok(())
}
