# rsblocks
[<img alt="github" src="https://img.shields.io/static/v1?label=github&message=rsblocks&color=acb0d0&logo=Github&style=flat-square&logoColor=a9b1d6" height="20">](https://github.com/MustafaSalih1993/rsblocks)
[<img alt="crates" src="https://img.shields.io/crates/v/rsblocks?logo=rust&logoColor=a9b1d6&style=flat-square&color=fc8d62" height="20">](https://crates.io/crates/rsblocks)


A multi threaded fast status bar for dwm window manager written in **Rust** 🦀
<p>
<img align="center" src="./screenshots/2.png"/>
</p><br/>

## Features
* Multi Threads
* Time/Date
* Net Usage
* Memory Usage
* Disk Usage
* Weather Temperature
* Sound Volume
* Battery Percentage
* Cpu Temperature
* Load Average
* Public Ip Address
* Bitcoin Price
* Uptime
* Mpd Current Song
* Spotify Current Song
* Easy to configure with `rsblocks.yml` file


## Notes
* This tool is still in development stage.
* currently supports only linux.


## Build Requirements
* [Libdbus](https://dbus.freedesktop.org/releases/dbus/) 1.6 or higher as a requirement to spotify.

On ubuntu you can do:
```sh
sudo apt install libdbus-1-dev pkg-config
```


## Cargo Installation
You can install the binary crate directly
```sh
cargo install rsblocks
```

## Manual Installation
You can clone the repo and build from the source code
```sh
git clone https://github.com/mustafasalih1993/rsblocks
```
build with **cargo**
```sh
cargo build --release
```
move the executable somewhere in your **PATH** (assuming you are in the root dir of the project)
```sh
mv ./target/release/rsblocks /usr/local/bin
```

you good to go now and can run `rsblocks` from your terminal or put that in your `.xinitrc`

## Configuration
#### Notes:
* **rsblocks** will try to read the file `$HOME/.config/rsblocks/rsblocks.yml`, if it does not exist, it will load the defaults.
* **rsblocks** will read the configuration file **only** on startup, which means you have to kill it and start it again if you updated your `rsblocks.yml` file.

create the directory
```sh
mkdir ~/.config/rsblocks
```

copy the [template](./rsblocks.yml) to the new config directory (assuming you are in the root dir of the repo)
```sh
cp ./rsblocks.yml ~/.config/rsblocks/rsblocks.yml
```


## Contributions
All Contributions are welcome.

## Credits
* [wttr.in](https://github.com/chubin/wttr.in) for using their weather API

## License
[MIT](./LICENSE)
