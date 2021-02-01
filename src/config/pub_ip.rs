use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PubIp {
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
}

impl Default for PubIp {
    fn default() -> Self {
        PubIp {
            icon: String::from(""),
            enabled: false,
            delay: 120.0,
        }
    }
}
