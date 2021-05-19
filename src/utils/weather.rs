use crate::config::CONFIG;
use crate::types::ThreadsData;

// will make a GET request from wttr.in
pub async fn get_weather() -> ThreadsData {
    let format = if CONFIG.weather.format.is_empty() {
        String::from("%l:+%t")
    } else {
        CONFIG.weather.format.clone()
    };

    let url = format!("http://wttr.in/{}?format=\"{}", CONFIG.weather.city, format);
    let err_string = String::from("Error");
    let res = match minreq::get(url).send() {
        Ok(resp) => match resp.as_str() {
            Ok(res_str) => res_str.trim_matches('"').to_string(),
            Err(_) => err_string,
        },
        Err(_) => err_string,
    };

    let data = format!("  {}  {}  {}", CONFIG.weather.icon, res, CONFIG.seperator);
    ThreadsData::Weather(data)
}
