use crate::blockmanager::*;
use crate::config::CONFIG;
use crate::types::*;
use crate::utils::*;
use async_std::channel::{unbounded, Sender};
use async_std::task;
use async_std::task::sleep;
use futures::future;
use futures::stream::StreamExt;
use std::future::Future;
use std::time::Duration;

async fn init_block<F, Fut>(tx: Sender<ThreadsData>, block: F, delay: f64)
where
    F: Fn() -> Fut,
    Fut: Future<Output = ThreadsData>,
{
    loop {
        let _ = tx.send(block().await).await;
        sleep(Duration::from_secs_f64(delay)).await;
    }
}

pub async fn run(mut blocks: BlockManager) {
    let (tx, rx) = unbounded();

    // loadavrage task
    if CONFIG.loadavg.enabled {
        let b = init_block(tx.clone(), load_average::get_load_avg, CONFIG.loadavg.delay);
        task::spawn(b);
    }
    // public ip task
    if CONFIG.pub_ip.enabled {
        let b = init_block(tx.clone(), pub_ip::get_pub_ip, CONFIG.pub_ip.delay);
        task::spawn(b);
    }

    // local ip task
    if CONFIG.local_ip.enabled {
        let b = init_block(tx.clone(), local_ip::get_local_ip, CONFIG.local_ip.delay);
        task::spawn(b);
    }

    // spotify task
    if CONFIG.spotify.enabled {
        let b = init_block(tx.clone(), spotify::get_spotify, CONFIG.spotify.delay);
        task::spawn(b);
    }

    // mpd task
    if CONFIG.mpd.enabled {
        let b = init_block(tx.clone(), mpd::get_mpd_current, CONFIG.mpd.delay);
        task::spawn(b);
    }

    // volume task
    if CONFIG.volume.enabled {
        let b = init_block(tx.clone(), volume::get_volume, CONFIG.volume.delay);
        task::spawn(b);
    }

    // Disk task
    if CONFIG.disk.enabled {
        let b = init_block(tx.clone(), disk::get_disk, CONFIG.disk.delay);
        task::spawn(b);
    }

    // Memory task
    if CONFIG.memory.enabled {
        let b = init_block(tx.clone(), memory::get_memory, CONFIG.memory.delay);
        task::spawn(b);
    }

    // Weather task
    if CONFIG.weather.enabled {
        let b = init_block(tx.clone(), weather::get_weather, CONFIG.weather.delay);
        task::spawn(b);
    }

    // Battery task
    if CONFIG.battery.enabled {
        let b = init_block(tx.clone(), battery::get_battery, CONFIG.battery.delay);
        task::spawn(b);
    }

    // Cpu temperature task
    if CONFIG.cpu_temperature.enabled {
        let b = init_block(tx.clone(), cpu::get_cpu_temp, CONFIG.cpu_temperature.delay);
        task::spawn(b);
    }

    // Uptime task
    if CONFIG.uptime.enabled {
        let b = init_block(tx.clone(), uptime::get_uptime, CONFIG.uptime.delay);
        task::spawn(b);
    }

    // BTC task
    if CONFIG.bitcoins.enabled {
        let b = init_block(tx.clone(), bitcoins::get_price, CONFIG.bitcoins.delay);
        task::spawn(b);
    }

    // net speed task
    if CONFIG.netspeed.enabled {
        let b = init_block(tx.clone(), netspeed::get_netspeed, 0.);
        task::spawn(b);
    }

    // Time task
    let b = init_block(tx, time::get_time, CONFIG.time.delay);
    task::spawn(b);

    rx.for_each(|data| {
        blocks.update(data);
        future::ready(())
    })
    .await;
}
