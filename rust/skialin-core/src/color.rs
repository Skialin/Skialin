pub type Color = u32;

pub const fn argb(a: u8, r: u8, g: u8, b: u8) -> Color {
    ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

pub const fn rgb(r: u8, g: u8, b: u8) -> Color {
    argb(0xff, r, g, b)
}

pub const BLACK: Color = argb(0xff, 0, 0, 0);
pub const WHITE: Color = argb(0xff, 0xff, 0xff, 0xff);
pub const TRANSPARENT: Color = argb(0, 0, 0, 0);
pub const RED: Color = argb(0xff, 0xff, 0, 0);
pub const GREEN: Color = argb(0xff, 0, 0xff, 0);
pub const BLUE: Color = argb(0xff, 0, 0, 0xff);
