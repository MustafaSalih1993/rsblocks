use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalIp {
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
    pub interface: String,
}

impl Default for LocalIp {
    fn default() -> Self {
        LocalIp {
            icon: String::from(""),
            enabled: false,
            delay: 120.0,
            interface: String::from("wlan0"),
        }
    }
}
