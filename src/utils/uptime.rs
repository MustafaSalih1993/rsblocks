use crate::config::CONFIG;
use crate::types::ThreadsData;
use nix::time::clock_gettime;

pub async fn get_uptime() -> ThreadsData {
    let (_, hour, minutes, seconds) = get_uptime_data();
    let uptime = if hour > 0 {
        format!("{}:{}:{}", hour, minutes, seconds)
    } else {
        format!("{} min, {} sec", minutes, seconds)
    };
    let result = format!("  {}  {}  {}", CONFIG.uptime.icon, uptime, CONFIG.seperator);
    ThreadsData::Uptime(result)
}

// This helper function will use the system call clock_gettime
// it will return a tuple of (days, hours, minutes, seconds)
fn get_uptime_data() -> (i64, i64, i64, i64) {
    let mut uptime = clock_gettime(nix::time::ClockId::CLOCK_MONOTONIC)
        .unwrap()
        .tv_sec();

    if uptime > 60 {
        uptime += 30;
    }
    let days = uptime / 86400;
    uptime %= 86400;
    let hours = uptime / 3600;
    uptime %= 3600;
    let minutes = uptime / 60;
    let seconds = uptime % 60;
    (days, hours, minutes, seconds)
}
