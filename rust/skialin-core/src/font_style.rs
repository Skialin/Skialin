#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Slant {
    Upright,
    Italic,
    Oblique,
}

impl From<i32> for Slant {
    fn from(value: i32) -> Self {
        match value {
            1 => Slant::Italic,
            2 => Slant::Oblique,
            _ => Slant::Upright,
        }
    }
}

impl From<Slant> for i32 {
    fn from(value: Slant) -> Self {
        match value {
            Slant::Upright => 0,
            Slant::Italic => 1,
            Slant::Oblique => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FontStyle {
    pub weight: i32,
    pub width: i32,
    pub slant: Slant,
}

impl FontStyle {
    pub const fn new(weight: i32, width: i32, slant: Slant) -> Self {
        FontStyle { weight, width, slant }
    }

    pub const fn normal() -> Self {
        Self::new(400, 5, Slant::Upright)
    }

    pub const fn bold() -> Self {
        Self::new(700, 5, Slant::Upright)
    }

    pub const fn italic() -> Self {
        Self::new(400, 5, Slant::Italic)
    }

    pub const fn bold_italic() -> Self {
        Self::new(700, 5, Slant::Italic)
    }
}

impl Default for FontStyle {
    fn default() -> Self {
        Self::normal()
    }
}
