use crate::bluetooth::BTBatteryLevel;
use crate::dygma::DygmaBatteryInfo;
use anyhow::{Error, Result};
use serde::Serialize;
use tokio::io::{stdout, AsyncWriteExt};

#[derive(Serialize)]
pub(crate) struct Report {
    pub(crate) battery_level: String,
    pub(crate) battery_status: String,
    pub(crate) name: String,
}

impl From<DygmaBatteryInfo> for Report {
    fn from(value: DygmaBatteryInfo) -> Self {
        let battery_level = format!("{}/{}", value.left_level, value.right_level);
        Report {
            name: value.name,
            battery_level,
            battery_status: value.left_status.to_string(),
        }
    }
}

impl From<BTBatteryLevel> for Report {
    fn from(value: BTBatteryLevel) -> Self {
        Report {
            name: value.name,
            battery_level: value.level.to_string(),
            battery_status: value.status.to_string(),
        }
    }
}


impl ToString for Report {
    fn to_string(&self) -> String {
        format!("{}: {}\n", self.name, self.battery_level)
    }
}


pub(crate) async fn print_reports(reports: Vec<Report>, as_json: &bool) -> Result<(), Error> {
    let mut output = String::new();
    if *as_json {
        output = serde_json::to_string(&reports)?;
    } else {
        for report in reports {
            output.push_str(&report.to_string());
        }
    }
    stdout().write_all(output.as_bytes()).await?;
    Ok(())
}
