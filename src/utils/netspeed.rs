use crate::config::CONFIG;
use crate::types::ThreadsData;
use std::fs::read_to_string;
use async_std::task::sleep;
use std::time::Duration;

pub async fn get_netspeed() -> ThreadsData {
    let tx1: u64 = parse_speed_file("tx_bytes");
    let rx1: u64 = parse_speed_file("rx_bytes");
    sleep(Duration::from_secs(1)).await;
    let tx2: u64 = parse_speed_file("tx_bytes");
    let rx2: u64 = parse_speed_file("rx_bytes");

    let tx_bps = tx2 - tx1;
    let rx_bps = rx2 - rx1;

    let tx = calculate(tx_bps);
    let rx = calculate(rx_bps);

    let data = format!(
        "  {}  {}  {}  {}  {}",
        CONFIG.netspeed.recieve_icon, rx, CONFIG.netspeed.transmit_icon, tx, CONFIG.seperator
    );
    ThreadsData::NetSpeed(data)
}

fn parse_speed_file(pth: &str) -> u64 {
    let base_path = format!("/sys/class/net/{}/statistics/", CONFIG.netspeed.interface);
    let x: u64 = read_to_string(base_path + pth)
        .unwrap()
        .trim()
        .parse::<u64>()
        .unwrap();

    x
}

fn calculate(speed: u64) -> String {
    let lookup = ["B", "kB", "MB"];
    let mut speed = speed as f64;
    let mut idx = 0;
    while speed >= 1024.0 && idx < lookup.len() {
        speed /= 1024.0;
        idx += 1;
    }
    format!("{:.1} {}", speed, lookup[idx])
}
