use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Nordvpn {
    pub icon: String,
    pub enabled: bool,
    pub showip: bool,
    pub delay: f64,
}

impl Default for Nordvpn {
    fn default() -> Self {
        Nordvpn {
            icon: String::from("撚"),
            enabled: false,
            showip: false,
            delay: 0.17,
        }
    }
}
