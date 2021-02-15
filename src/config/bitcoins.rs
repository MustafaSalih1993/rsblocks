use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitCoins {
    pub icon: String,
    pub symbol: String,
    pub enabled: bool,
    pub delay: f64,
}

impl Default for BitCoins {
    fn default() -> Self {
        BitCoins {
            icon: String::from(""),
            symbol: String::from("BTC-USD"),
            enabled: false,
            delay: 120.0,
        }
    }
}
