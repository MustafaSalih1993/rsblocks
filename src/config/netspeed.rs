use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetSpeed {
    pub transmit_icon: String,
    pub recieve_icon: String,
    pub interface: String,
    pub enabled: bool,
}

impl Default for NetSpeed {
    fn default() -> Self {
        NetSpeed {
            transmit_icon: String::from(""),
            recieve_icon: String::from(""),
            interface: String::from("wlan0"),
            enabled: false,
        }
    }
}
