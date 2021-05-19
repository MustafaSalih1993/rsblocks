use breadx::{display::*, window::Window};

pub struct BlockManager {
    pub disp: Display<name::NameConnection>,
    pub root: Window,
}

impl BlockManager {
    pub fn new() -> Self {
        let disp = Display::create(None, None).expect("Failed to create x11 connection");
        let root = disp.default_screen().root;
        Self { disp, root }
    }

    pub fn update(&mut self, bar: &[String]) {
        let mut x = String::new();
        for i in bar.iter() {
            x.push_str(i.as_str());
        }

        self
            .root
            .set_title(&mut self.disp, &x)
            .expect("Failed to set title of root");
    }
}

impl Default for BlockManager {
    fn default() -> Self {
        Self::new()
    }
}
