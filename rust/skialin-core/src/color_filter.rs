use crate::{sys, BlendMode, Color};

pub struct ColorFilter(pub(crate) *mut sys::SkColorFilter);

impl ColorFilter {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::SkColorFilter) -> Option<Self> {
        (!ptr.is_null()).then_some(ColorFilter(ptr))
    }

    pub fn blend(color: Color, mode: BlendMode) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ColorFilter_Blend(color, mode.into())) }
    }

    /// `row_major_20` is a 4x5 row-major color matrix.
    pub fn matrix(row_major_20: &[f32; 20], clamp: bool) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ColorFilter_Matrix(row_major_20.as_ptr(), clamp)) }
    }

    pub fn compose(outer: &ColorFilter, inner: &ColorFilter) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ColorFilter_Compose(outer.0, inner.0)) }
    }

    pub fn lerp(t: f32, dst: &ColorFilter, src: &ColorFilter) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ColorFilter_Lerp(t, dst.0, src.0)) }
    }
}

impl Drop for ColorFilter {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_ColorFilter_unref(self.0) };
    }
}
