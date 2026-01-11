use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    SolarPanel,
    EVCharger,
    Sauna,
    HeatPump,
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoTMessage {
    pub device_id: String,
    pub device_type: DeviceType,
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub unit: String,
    pub location: String, // "lat,lon" or "Region"
}

// Validation or utility methods can go here
impl IoTMessage {
    pub fn new(
        device_id: String,
        device_type: DeviceType,
        value: f64,
        unit: String,
        location: String,
    ) -> Self {
        Self {
            device_id,
            device_type,
            timestamp: Utc::now(),
            value,
            unit,
            location,
        }
    }
}
