use crate::config::CONFIG;

pub fn get_pub_ip() -> String {
    let url = format!("http://api.ipify.org");
    let err_string = String::from("Error");
    let res = match minreq::get(url).send() {
        Ok(resp) => match resp.as_str() {
            Ok(res_str) => res_str.trim().to_string(),
            Err(_) => err_string,
        },
        Err(_) => err_string,
    };

    format!("  {}  {}  {}", CONFIG.pub_ip.icon, res, CONFIG.seperator)
}
