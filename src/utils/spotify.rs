use crate::config::CONFIG;
use crate::types::ThreadsData;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use dbus::{arg, blocking::Connection};
use std::time::Duration;

// getting spotify current artist and title.
pub async fn get_spotify() -> ThreadsData {
    let empty_data = ThreadsData::Spotify(String::from(""));
    let conn = match Connection::new_session() {
        Ok(conn) => conn,
        _ => return empty_data,
    };

    let p = conn.with_proxy(
        "org.mpris.MediaPlayer2.spotify",
        "/org/mpris/MediaPlayer2",
        Duration::from_millis(5000),
    );

    let metadata: arg::PropMap = match p.get("org.mpris.MediaPlayer2.Player", "Metadata") {
        Ok(data) => data,
        _ => return empty_data,
    };

    let title: Option<&String> = arg::prop_cast(&metadata, "xesam:title");
    let artist: Option<&Vec<String>> = arg::prop_cast(&metadata, "xesam:artist");

    let title = match title {
        Some(title) => {
            if title.len() > 30 {
                &title[..30]
            } else {
                title.as_str()
            }
        }
        _ => "",
    };

    let artist = match artist {
        Some(artist_vec) => match artist_vec.first() {
            Some(name) => {
                if name.len() > 30 {
                    &name[..30]
                } else {
                    name.as_str()
                }
            }
            _ => "",
        },
        None => "",
    };

    let data = format!(
        "  {}  {} - {}  {}",
        CONFIG.spotify.icon, artist, title, CONFIG.seperator
    );
    ThreadsData::Spotify(data)
}
