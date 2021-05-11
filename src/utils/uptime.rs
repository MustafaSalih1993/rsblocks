use crate::config::CONFIG;
use crate::types::ThreadsData;
use chrono::Duration;
use nix::sys::sysinfo;

pub fn get_uptime() -> ThreadsData {
    let duration = sysinfo::sysinfo().unwrap().uptime();
    let uptime_sec = Duration::from_std(duration).unwrap().num_seconds();

    let hour = uptime_sec / 3600;
    let rem = uptime_sec - hour * 3600;
    let minutes = rem / 60;

    let uptime = if hour > 0 {
        format!("{}:{}", hour, minutes)
    } else {
        format!("{} min", minutes)
    };
    let result = format!("  {}  {}  {}", CONFIG.uptime.icon, uptime, CONFIG.seperator);
    ThreadsData::Uptime(result)
}
