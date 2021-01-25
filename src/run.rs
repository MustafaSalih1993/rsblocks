use crate::config::CONFIG;
use crate::types::*;
use crate::utils::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run(mut blocks: Blocks) {
    let (tx, rx) = mpsc::channel();

    // loadavrage thread
    if CONFIG.loadavg.enabled {
        let loadavg_tx = tx.clone();
        thread::spawn(move || loop {
            let loadavg_data = ThreadsData::LoadAvg(load_average::get_load_avg());
            loadavg_tx.send(loadavg_data).unwrap();
            thread::sleep(Duration::from_secs_f64(CONFIG.loadavg.delay))
        });
    }
    // spotify thread
    if CONFIG.spotify.enabled {
        let spotify_tx = tx.clone();
        thread::spawn(move || loop {
            let spotify_data = ThreadsData::Spotify(spotify::get_spotify());
            spotify_tx.send(spotify_data).unwrap();
            thread::sleep(Duration::from_secs_f64(CONFIG.spotify.delay))
        });
    }

    // mpd thread
    if CONFIG.mpd.enabled {
        let mpd_tx = tx.clone();
        thread::spawn(move || loop {
            let mpd_data = ThreadsData::Mpd(mpd::get_mpd_current());
            mpd_tx.send(mpd_data).unwrap();
            thread::sleep(Duration::from_secs_f64(CONFIG.mpd.delay))
        });
    }

    // volume thread
    if CONFIG.volume.enabled {
        let volume_tx = tx.clone();
        thread::spawn(move || loop {
            let vol_data = ThreadsData::Sound(volume::get_volume());
            volume_tx.send(vol_data).unwrap();
            thread::sleep(Duration::from_secs_f64(CONFIG.volume.delay))
        });
    }

    // net speed thread
    if CONFIG.netspeed.enabled {
        let net_tx = tx.clone();
        thread::spawn(move || loop {
            // get_netspeed will sleep inside the function
            let net_data = ThreadsData::NetSpeed(netspeed::get_netspeed());
            net_tx.send(net_data).unwrap();
        });
    }

    // Disk thread
    if CONFIG.disk.enabled {
        let disk_tx = tx.clone();
        thread::spawn(move || loop {
            let disk_data = ThreadsData::Disk(disk::get_disk());
            disk_tx.send(disk_data).unwrap();
            thread::sleep(Duration::from_secs_f64(CONFIG.disk.delay))
        });
    }

    // Memory thread
    if CONFIG.memory.enabled {
        let memory_tx = tx.clone();
        thread::spawn(move || loop {
            let memory_data = ThreadsData::Memory(memory::get_memory().unwrap());
            memory_tx.send(memory_data).unwrap();
            thread::sleep(Duration::from_secs_f64(CONFIG.memory.delay))
        });
    }

    // Weather thread
    if CONFIG.weather.enabled {
        let weather_tx = tx.clone();
        thread::spawn(move || loop {
            let weather_data = ThreadsData::Weather(weather::get_weather());
            weather_tx.send(weather_data).unwrap();
            thread::sleep(Duration::from_secs_f64(CONFIG.weather.delay))
        });
    }

    // Battery thread
    if CONFIG.battery.enabled {
        let battery_tx = tx.clone();
        thread::spawn(move || loop {
            let battery_data = ThreadsData::Battery(battery::get_battery().unwrap());
            battery_tx.send(battery_data).unwrap();
            thread::sleep(Duration::from_secs_f64(CONFIG.battery.delay))
        });
    }

    // Cpu temperature thread
    if CONFIG.cpu_temperature.enabled {
        let cpu_temp_tx = tx.clone();
        thread::spawn(move || loop {
            let cpu_temp_data = ThreadsData::CpuTemp(cpu::get_cpu_temp().unwrap());
            cpu_temp_tx.send(cpu_temp_data).unwrap();
            thread::sleep(Duration::from_secs_f64(CONFIG.cpu_temperature.delay))
        });
    }

    // Uptime thread
    if CONFIG.uptime.enabled {
        let uptime_tx = tx.clone();
        thread::spawn(move || loop {
            let uptime_data = ThreadsData::Uptime(uptime::get_uptime().unwrap());
            uptime_tx.send(uptime_data).unwrap();
            thread::sleep(Duration::from_secs_f64(CONFIG.uptime.delay))
        });
    }

    // Time thread
    {
        let time_tx = tx;
        thread::spawn(move || loop {
            let time_data = ThreadsData::Time(time::get_time());
            time_tx.send(time_data).unwrap();
            thread::sleep(Duration::from_secs_f64(CONFIG.time.delay))
        });
    }

    //Main
    {
        // NOTE: order matters to the final format
        let mut bar: Vec<String> = vec![String::from(""); 12];
        //iterating the values recieved from the threads
        for data in rx {
            match data {
                ThreadsData::Spotify(x) => bar[0] = x,
                ThreadsData::Mpd(x) => bar[1] = x,
                ThreadsData::Sound(x) => bar[2] = x,
                ThreadsData::Weather(x) => bar[3] = x,
                ThreadsData::NetSpeed(x) => bar[4] = x,
                ThreadsData::Disk(x) => bar[5] = x,
                ThreadsData::Memory(x) => bar[6] = x,
                ThreadsData::CpuTemp(x) => bar[7] = x,
                ThreadsData::LoadAvg(x) => bar[8] = x,
                ThreadsData::Battery(x) => bar[9] = x,
                ThreadsData::Uptime(x) => bar[10] = x,
                ThreadsData::Time(x) => bar[11] = x,
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
