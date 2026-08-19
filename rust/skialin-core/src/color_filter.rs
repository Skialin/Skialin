use crate::{sys, BlendMode, Color};

pub struct ColorFilter(pub(crate) *mut sys::SkColorFilter);

impl ColorFilter {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::SkColorFilter) -> Option<Self> {
        (!ptr.is_null()).then_some(ColorFilter(ptr))
    }

    pub fn blend(color: Color, mode: BlendMode) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ColorFilter_Blend(color, mode.into())) }
    }

    pub fn matrix(row_major_20: &[f32; 20], clamp: bool) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ColorFilter_Matrix(row_major_20.as_ptr(), clamp)) }
    }

    pub fn compose(outer: &ColorFilter, inner: &ColorFilter) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ColorFilter_Compose(outer.0, inner.0)) }
    }

    pub fn lerp(t: f32, dst: &ColorFilter, src: &ColorFilter) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ColorFilter_Lerp(t, dst.0, src.0)) }
    }

    pub fn hsla_matrix(row_major_20: &[f32; 20]) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ColorFilter_HSLAMatrix(row_major_20.as_ptr())) }
    }

    pub fn linear_to_srgb_gamma() -> Self {
        unsafe { Self::from_raw(sys::skialin_bridge_ColorFilter_LinearToSRGBGamma()) }.expect("LinearToSRGBGamma never returns null")
    }

    pub fn srgb_to_linear_gamma() -> Self {
        unsafe { Self::from_raw(sys::skialin_bridge_ColorFilter_SRGBToLinearGamma()) }.expect("SRGBToLinearGamma never returns null")
    }

    pub fn table(table_256: &[u8; 256]) -> Self {
        unsafe { Self::from_raw(sys::skialin_bridge_ColorFilter_Table(table_256.as_ptr())) }.expect("Table never returns null")
    }

    pub fn table_argb(a: Option<&[u8; 256]>, r: Option<&[u8; 256]>, g: Option<&[u8; 256]>, b: Option<&[u8; 256]>) -> Self {
        let ptr = |t: Option<&[u8; 256]>| t.map_or(std::ptr::null(), |t| t.as_ptr());
        unsafe { Self::from_raw(sys::skialin_bridge_ColorFilter_TableARGB(ptr(a), ptr(r), ptr(g), ptr(b))) }.expect("TableARGB never returns null")
    }

    pub fn lighting(mul: Color, add: Color) -> Self {
        unsafe { Self::from_raw(sys::skialin_bridge_ColorFilter_Lighting(mul, add)) }.expect("Lighting never returns null")
    }

    pub fn high_contrast(grayscale: bool, invert_style: i32, contrast: f32) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ColorFilter_HighContrast(grayscale, invert_style, contrast)) }
    }

    pub fn luma() -> Self {
        unsafe { Self::from_raw(sys::skialin_bridge_ColorFilter_Luma()) }.expect("Luma never returns null")
    }
}

impl Drop for ColorFilter {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_ColorFilter_unref(self.0) };
    }
}
