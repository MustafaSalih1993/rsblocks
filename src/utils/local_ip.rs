use crate::config::CONFIG;
use crate::types::ThreadsData;

pub async fn get_local_ip() -> ThreadsData {
    let addrs = nix::ifaddrs::getifaddrs().unwrap();
    let mut ip = String::new();
    for ifaddr in addrs {
        match ifaddr.address {
            Some(address) => {
                if ifaddr.interface_name == CONFIG.local_ip.interface {
                    match address.family() {
                        nix::sys::socket::AddressFamily::Inet => {
                            ip = address.to_string().split(':').next().unwrap().to_string();
                            break;
                        }
                        _ => continue,
                    };
                }
            }
            None => continue,
        }
    }
    if ip.is_empty() {
        ip = String::from("Error!")
    }
    let data = format!("  {}  {}  {}", CONFIG.local_ip.icon, ip, CONFIG.seperator);
    ThreadsData::LocalIp(data)
}
