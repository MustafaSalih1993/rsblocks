use crate::config::CONFIG;
use crate::types::ThreadsData;
use chrono::prelude::*;

pub async fn get_time() -> ThreadsData {
    let now = Local::now();

    let data = format!(
        "  {}  {}  {}",
        CONFIG.time.icon,
        now.format(&CONFIG.time.format),
        CONFIG.seperator
    );
    ThreadsData::Time(data)
}
