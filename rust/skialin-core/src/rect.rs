use crate::sys;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Rect {
    pub const fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Rect { left, top, right, bottom }
    }

    pub const fn from_wh(width: f32, height: f32) -> Self {
        Rect::new(0.0, 0.0, width, height)
    }

    pub fn width(&self) -> f32 {
        self.right - self.left
    }

    pub fn height(&self) -> f32 {
        self.bottom - self.top
    }
}

impl From<Rect> for sys::SkRect {
    fn from(r: Rect) -> Self {
        sys::SkRect { fLeft: r.left, fTop: r.top, fRight: r.right, fBottom: r.bottom }
    }
}

impl From<sys::SkRect> for Rect {
    fn from(r: sys::SkRect) -> Self {
        Rect { left: r.fLeft, top: r.fTop, right: r.fRight, bottom: r.fBottom }
    }
}
