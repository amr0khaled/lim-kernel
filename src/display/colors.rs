#[repr(C, packed)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub fn mono(v: u8) -> Self {
        Self { r: v, g: v, b: v }
    }
    pub fn ret(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}
