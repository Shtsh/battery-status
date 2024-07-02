use crate::bluetooth::BTBatteryLevel;
use crate::dygma::DygmaBatteryInfo;
use anyhow::{Context, Error, Result};
use std::io::{stdout, Write};

pub(crate) fn print_bt_battery_levels(levels: Vec<BTBatteryLevel>) -> Result<(), Error> {
    for level in levels {
        stdout()
            .write_all(format!("{}: {}\n", level.name, level.level).as_bytes())
            .context("Unable to write to stdout")?;
    }
    Ok(())
}

pub(crate) fn print_dygma_battery_levels(level: DygmaBatteryInfo) -> Result<(), Error> {
    stdout()
        .write_all(
            format!(
                "{}: {}/{}\n",
                level.name, level.left_level, level.right_level
            )
            .as_bytes(),
        )
        .context("Unable to write to stdout")?;
    Ok(())
}
