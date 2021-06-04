use crate::types::ThreadsData;
use breadx::{display::*, Window};

pub struct BlockManager {
    pub disp: Display<name::NameConnection>,
    blocks: Vec<String>,
    pub root: Window,
}

impl BlockManager {
    pub fn new() -> Self {
        let disp = Display::create(None, None).expect("Failed to create x11 connection");
        let root = disp.default_screen().root;
        Self {
            disp,
            blocks: vec![String::from(""); 15],
            root,
        }
    }

    pub fn update(&mut self, data: ThreadsData) {
        match data {
            ThreadsData::Spotify(x) => self.blocks[0] = x,
            ThreadsData::Mpd(x) => self.blocks[1] = x,
            ThreadsData::Sound(x) => self.blocks[2] = x,
            ThreadsData::Weather(x) => self.blocks[3] = x,
            ThreadsData::NetSpeed(x) => self.blocks[4] = x,
            ThreadsData::BitCoins(x) => self.blocks[5] = x,
            ThreadsData::PubIp(x) => self.blocks[6] = x,
            ThreadsData::LocalIp(x) => self.blocks[7] = x,
            ThreadsData::Disk(x) => self.blocks[8] = x,
            ThreadsData::Memory(x) => self.blocks[9] = x,
            ThreadsData::CpuTemp(x) => self.blocks[10] = x,
            ThreadsData::LoadAvg(x) => self.blocks[11] = x,
            ThreadsData::Battery(x) => self.blocks[12] = x,
            ThreadsData::Uptime(x) => self.blocks[13] = x,
            ThreadsData::Time(x) => self.blocks[14] = x,
        }
        let mut x = String::new();
        for i in self.blocks.iter() {
            x.push_str(i.as_str());
        }

        self.root
            .set_title(&mut self.disp, &x)
            .expect("Failed to set title");
    }
}

impl Default for BlockManager {
    fn default() -> Self {
        Self::new()
    }
}
