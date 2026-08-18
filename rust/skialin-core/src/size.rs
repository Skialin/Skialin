use crate::sys;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ISize {
    pub width: i32,
    pub height: i32,
}

impl ISize {
    pub const fn new(width: i32, height: i32) -> Self {
        ISize { width, height }
    }
}

impl From<ISize> for sys::SkISize {
    fn from(s: ISize) -> Self {
        sys::SkISize { fWidth: s.width, fHeight: s.height }
    }
}

impl From<sys::SkISize> for ISize {
    fn from(s: sys::SkISize) -> Self {
        ISize { width: s.fWidth, height: s.fHeight }
    }
}
