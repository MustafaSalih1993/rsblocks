use crate::config::CONFIG;
use crate::types::ThreadsData;
use std::process::Command;

/*
 * Nordvpn module
*/
pub async fn get_nordvpn() -> ThreadsData {

    let output = Command::new("nordvpn")
        .arg("status")
        .output()
        .expect("Failed to execute command");

    let mut server = String::new();
    let mut ip = String::new();
    let mut country = String::new();
    
    if CONFIG.nordvpn.showip {
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if line.starts_with("Current server: ") {
                server = line[16..21].to_string();
            }
            if line.starts_with("Server IP: ") {
                ip = line[11..].to_string();
            }
        }
    }else{
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            /* For some reason, getting line starting with Status fails,
             * Current server line is used to determine wether client
             * is connected or not*/
            if line.starts_with("Current server: ") {
                server = line[16..21].to_string();
            }
            if line.starts_with("Country: ") {
                country = line[8..].to_string();
            }
        }
    }

    let result: String;
    /* Check if client is connected */
    if server != "" {
        if CONFIG.nordvpn.showip{
            result = format!(
                " {} {}@{} {}",
                CONFIG.nordvpn.icon, ip, server, CONFIG.seperator);
        }else{
            result = format!(
                " {} {} {}",
                CONFIG.nordvpn.icon, country, CONFIG.seperator);
        }
    } else {
        result = format!(
            " {} {} {}",
            CONFIG.nordvpn.icon, "Disconnected", CONFIG.seperator);
    }
    ThreadsData::Nordvpn(result)
}
