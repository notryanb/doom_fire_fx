#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
    pub alpha: u32,
}

impl Pixel {
    pub fn is_black(self) -> bool {
        self.red <= 0x07 && self.green <= 0x07 && self.blue <= 0x07
    }
}

