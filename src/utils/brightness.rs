use crate::config::CONFIG;
use crate::types::ThreadsData;
use std::fs::read_to_string;
use std::path::Path;

// getting brightness
pub async fn get_brightness() -> ThreadsData {
    let brightness_path = Path::new(&CONFIG.brightness.path);
    if !brightness_path.exists() {
        return ThreadsData::Brightness(String::from("brightness path not found"));
    };
    let current_brightness = match read_to_string(Path::new(brightness_path).join("brightness")) {
        Ok(brightness) => brightness.trim().to_owned().parse::<f32>().unwrap(),
        _ => return ThreadsData::Brightness(String::from("error reading current brightness")),
    };
    let max_brightness = match read_to_string(Path::new(brightness_path).join("max_brightness")) {
        Ok(brightness) => brightness.trim().to_owned().parse::<f32>().unwrap(),
        _ => return ThreadsData::Brightness(String::from("error reading max brightness")),
    };

    let value = (current_brightness / max_brightness) * 100.0;

    let result = format!(
        "  {}  {:.0}%  {}",
        CONFIG.brightness.icon, value, CONFIG.seperator
    );
    ThreadsData::Brightness(result)
}
