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

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct IRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl IRect {
    pub const fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        IRect { left, top, right, bottom }
    }

    pub const fn from_wh(width: i32, height: i32) -> Self {
        IRect::new(0, 0, width, height)
    }

    pub fn width(&self) -> i32 {
        self.right - self.left
    }

    pub fn height(&self) -> i32 {
        self.bottom - self.top
    }
}

impl From<IRect> for sys::SkIRect {
    fn from(r: IRect) -> Self {
        sys::SkIRect { fLeft: r.left, fTop: r.top, fRight: r.right, fBottom: r.bottom }
    }
}

impl From<sys::SkIRect> for IRect {
    fn from(r: sys::SkIRect) -> Self {
        IRect { left: r.fLeft, top: r.fTop, right: r.fRight, bottom: r.fBottom }
    }
}
