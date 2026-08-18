use crate::sys;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlurStyle {
    Normal,
    Solid,
    Outer,
    Inner,
}

impl From<BlurStyle> for i32 {
    fn from(style: BlurStyle) -> Self {
        match style {
            BlurStyle::Normal => 0,
            BlurStyle::Solid => 1,
            BlurStyle::Outer => 2,
            BlurStyle::Inner => 3,
        }
    }
}

pub struct MaskFilter(pub(crate) *mut sys::SkMaskFilter);

impl MaskFilter {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::SkMaskFilter) -> Option<Self> {
        (!ptr.is_null()).then_some(MaskFilter(ptr))
    }

    pub fn blur(style: BlurStyle, sigma: f32, respect_ctm: bool) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_MaskFilter_MakeBlur(style.into(), sigma, respect_ctm)) }
    }
}

impl Drop for MaskFilter {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_MaskFilter_unref(self.0) };
    }
}
