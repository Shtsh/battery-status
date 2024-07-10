#[derive(Debug)]
pub enum BatteryStatus {
    Charging,
    Charged,
    Discharging,
    Unknown,
}

impl ToString for BatteryStatus {
    fn to_string(&self) -> String {
        match self {
            BatteryStatus::Charging => "Charging".into(),
            BatteryStatus::Charged => "Charged".into(),
            BatteryStatus::Discharging => "Discharging".into(),
            BatteryStatus::Unknown => "Unknown".into(),
        }
    }
}

