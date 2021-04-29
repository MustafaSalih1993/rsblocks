use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Battery {
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
}

impl Default for Battery {
    fn default() -> Self {
        Battery {
            icon: String::from(""),
            enabled: false,
            delay: 120.0,
        }
    }
}
