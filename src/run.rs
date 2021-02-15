use crate::config::CONFIG;
use crate::types::*;
use crate::utils::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn spawn_thread_loop<F>(tx: std::sync::mpsc::Sender<ThreadsData>, data: F, delay: f64)
where
    F: Fn() -> ThreadsData + Send + 'static,
{
    thread::spawn(move || loop {
        tx.send(data()).unwrap();
        thread::sleep(Duration::from_secs_f64(delay));
    });
}

pub fn run(mut blocks: Blocks) {
    let (tx, rx) = mpsc::channel();

    // loadavrage thread
    if CONFIG.loadavg.enabled {
        spawn_thread_loop(tx.clone(), load_average::get_load_avg, CONFIG.loadavg.delay);
    }
    // public ip thread
    if CONFIG.pub_ip.enabled {
        spawn_thread_loop(tx.clone(), pub_ip::get_pub_ip, CONFIG.pub_ip.delay);
    }

    // spotify thread
    if CONFIG.spotify.enabled {
        spawn_thread_loop(tx.clone(), spotify::get_spotify, CONFIG.spotify.delay);
    }

    // mpd thread
    if CONFIG.mpd.enabled {
        spawn_thread_loop(tx.clone(), mpd::get_mpd_current, CONFIG.mpd.delay);
    }

    // volume thread
    if CONFIG.volume.enabled {
        spawn_thread_loop(tx.clone(), volume::get_volume, CONFIG.volume.delay);
    }

    // Disk thread
    if CONFIG.disk.enabled {
        spawn_thread_loop(tx.clone(), disk::get_disk, CONFIG.disk.delay);
    }

    // Memory thread
    if CONFIG.memory.enabled {
        spawn_thread_loop(tx.clone(), memory::get_memory, CONFIG.memory.delay);
    }

    // Weather thread
    if CONFIG.weather.enabled {
        spawn_thread_loop(tx.clone(), weather::get_weather, CONFIG.weather.delay);
    }

    // Battery thread
    if CONFIG.battery.enabled {
        spawn_thread_loop(tx.clone(), battery::get_battery, CONFIG.battery.delay);
    }

    // Cpu temperature thread
    if CONFIG.cpu_temperature.enabled {
        spawn_thread_loop(tx.clone(), cpu::get_cpu_temp, CONFIG.cpu_temperature.delay);
    }

    // Uptime thread
    if CONFIG.uptime.enabled {
        spawn_thread_loop(tx.clone(), uptime::get_uptime, CONFIG.uptime.delay);
    }

    // BTC thread
    if CONFIG.bitcoins.enabled {
        spawn_thread_loop(tx.clone(), bitcoins::get_price, CONFIG.bitcoins.delay);
    }

    // net speed thread
    // get_netspeed will sleep inside the function
    if CONFIG.netspeed.enabled {
        let net_tx = tx.clone();
        thread::spawn(move || loop {
            let net_data = netspeed::get_netspeed();
            net_tx.send(net_data).unwrap();
        });
    }

    // Time thread
    {
        spawn_thread_loop(tx, time::get_time, CONFIG.time.delay);
    }

    //Main
    {
        // NOTE: order matters to the final format

        let mut bar: Vec<String> = vec![String::from(""); 14];
        //iterating the values recieved from the threads
        for data in rx {
            match data {
                ThreadsData::Spotify(x) => bar[0] = x,
                ThreadsData::Mpd(x) => bar[1] = x,
                ThreadsData::Sound(x) => bar[2] = x,
                ThreadsData::Weather(x) => bar[3] = x,
                ThreadsData::NetSpeed(x) => bar[4] = x,
                ThreadsData::BitCoins(x) => bar[5] = x,
                ThreadsData::PubIp(x) => bar[6] = x,
                ThreadsData::Disk(x) => bar[7] = x,
                ThreadsData::Memory(x) => bar[8] = x,
                ThreadsData::CpuTemp(x) => bar[9] = x,
                ThreadsData::LoadAvg(x) => bar[10] = x,
                ThreadsData::Battery(x) => bar[11] = x,
                ThreadsData::Uptime(x) => bar[12] = x,
                ThreadsData::Time(x) => bar[13] = x,
            }

            // match ends here
            update(&bar, &mut blocks);
        }
    }
}

fn update(bar: &[String], blocks: &mut Blocks) {
    let mut x = String::new();
    for i in bar.iter() {
        x.push_str(i.as_str());
    }

    blocks
        .root
        .set_title(&mut blocks.disp, &x)
        .expect("Failed to set title of root");
}
