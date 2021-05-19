use crate::config::CONFIG;
use crate::types::ThreadsData;

use nix::libc::{c_double, c_int, getloadavg};

pub async fn get_load_avg() -> ThreadsData {
    let mut data: [c_double; 3] = [0f64; 3];
    unsafe { getloadavg(data.as_mut_ptr(), data.len() as c_int) };
    let [load, _, _] = data;
    let data = format!(
        "  {}  {:.2}  {}",
        CONFIG.loadavg.icon, load, CONFIG.seperator
    );

    ThreadsData::LoadAvg(data)
}
