#[derive(Debug, Clone)]
pub enum ThreadsData {
    Mpd(String),
    Sound(String),
    Disk(String),
    Memory(String),
    Time(String),
    Weather(String),
    Battery(String),
    CpuTemp(String),
    Uptime(String),
    Spotify(String),
    LoadAvg(String),
    NetSpeed(String),
    PubIp(String),
    LocalIp(String),
    BitCoins(String),
}

#[derive(Clone)]
pub struct Config {
    pub seperator: String,
    pub time: Time,
    pub memory: Memory,
    pub disk: Disk,
    pub volume: Volume,
    pub weather: Weather,
    pub battery: Battery,
    pub cpu_temperature: CpuTemp,
    pub uptime: Uptime,
    pub mpd: Mpd,
    pub spotify: Spotify,
    pub loadavg: LoadAvg,
    pub pub_ip: PubIp,
    pub local_ip: LocalIp,
    pub bitcoins: BitCoins,
}

#[derive(Clone)]
pub struct Time {
    pub format: String,
    pub icon: String,
    pub delay: f64,
}

#[derive(Clone)]
pub struct Memory {
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
}

#[derive(Clone)]
pub struct Disk {
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
}
#[derive(Clone)]
pub struct Volume {
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
    pub card: String,
}

#[derive(Clone)]
pub struct Weather {
    pub city: String,
    pub format: String,
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
}

#[derive(Clone)]
pub struct Battery {
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
}

#[derive(Clone)]
pub struct CpuTemp {
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
}

#[derive(Clone)]
pub struct Uptime {
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
}

#[derive(Clone)]
pub struct Mpd {
    pub icon: String,
    pub host: String,
    pub port: String,
    pub enabled: bool,
    pub delay: f64,
}

#[derive(Clone)]
pub struct Spotify {
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
}

#[derive(Clone)]
pub struct LoadAvg {
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
}

#[derive(Clone)]
pub struct NetSpeed {
    pub transmit_icon: String,
    pub recieve_icon: String,
    pub interface: String,
    pub enabled: bool,
}

#[derive(Clone)]
pub struct PubIp {
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
}

#[derive(Clone)]
pub struct LocalIp {
    pub icon: String,
    pub enabled: bool,
    pub delay: f64,
}

#[derive(Clone)]
pub struct BitCoins {
    pub icon: String,
    pub symbol: String,
    pub enabled: bool,
    pub delay: f64,
}
