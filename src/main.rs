mod arguments;
mod bluetooth;
mod dygma;
mod formatter;
mod permissions;

use anyhow::{Error, Result};
use clap::Parser;
use simplelog::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = arguments::Cli::parse();

    CombinedLogger::init(vec![TermLogger::new(
        cli.verbose.log_level_filter(),
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )])
    .unwrap();
    debug!("Checking permissions");
    permissions::check_permissions()?;
    if cli.bluetooth_support {
        info!("Bluetooth devices are enabled");
        let adapter = bluetooth::get_adapter().await?;
        let status = bluetooth::process_adapter(adapter).await?;
        formatter::print_bt_battery_levels(status).await?;
    }

    if cli.dygma_support {
        info!("Dygma devices are enabled");
        let devices = tokio::task::block_in_place(move || {
            return dygma::list_devices();
        })?;
        if devices.is_empty() {
            info!("No Dygma devices found");
        }
        for device in devices {
            let status = dygma::get_battery_info(device.port_name)?;
            formatter::print_dygma_battery_levels(status).await?;
        }
    }
    Ok(())
}
