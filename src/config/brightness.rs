use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Brightness {
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
    pub path: String,
}

impl Default for Brightness {
    fn default() -> Self {
        Brightness {
            icon: String::from(""),
            enabled: false,
            delay: 120.0,
            path: String::from("/sys/class/backlight/intel_backlight"),
        }
    }
}
