use crate::config::CONFIG;
use crate::types::ThreadsData;
use std::fs::read_to_string;

// getting cpu temperature
pub async fn get_cpu_temp() -> ThreadsData {
    let buf = match read_to_string("/sys/class/thermal/thermal_zone0/temp") {
        Ok(data) => data,
        _ => return ThreadsData::CpuTemp(String::from("Error reading temp")),
    };

    let value = buf.trim().parse::<f32>().unwrap();

    let result = format!(
        "  {}  {}°  {}",
        CONFIG.cpu_temperature.icon,
        value / 1000.0,
        CONFIG.seperator
    );
    ThreadsData::CpuTemp(result)
}
