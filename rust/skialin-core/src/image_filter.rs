use crate::{sys, Color, ColorFilter, Matrix, SamplingOptions, TileMode};

pub struct ImageFilter(pub(crate) *mut sys::SkImageFilter);

impl ImageFilter {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::SkImageFilter) -> Option<Self> {
        (!ptr.is_null()).then_some(ImageFilter(ptr))
    }

    fn input_ptr(input: Option<&ImageFilter>) -> *mut sys::SkImageFilter {
        input.map_or(std::ptr::null_mut(), |f| f.0)
    }

    pub fn blur(sigma_x: f32, sigma_y: f32, tile_mode: TileMode, input: Option<&ImageFilter>) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ImageFilter_Blur(sigma_x, sigma_y, tile_mode.into(), Self::input_ptr(input))) }
    }

    pub fn drop_shadow(dx: f32, dy: f32, sigma_x: f32, sigma_y: f32, color: Color, input: Option<&ImageFilter>) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ImageFilter_DropShadow(dx, dy, sigma_x, sigma_y, color, Self::input_ptr(input))) }
    }

    /// Renders the drop shadow without the input content, so callers can
    /// compose the shadow and input in their own filter graph.
    pub fn drop_shadow_only(dx: f32, dy: f32, sigma_x: f32, sigma_y: f32, color: Color, input: Option<&ImageFilter>) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ImageFilter_DropShadowOnly(dx, dy, sigma_x, sigma_y, color, Self::input_ptr(input))) }
    }

    pub fn offset(dx: f32, dy: f32, input: Option<&ImageFilter>) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ImageFilter_Offset(dx, dy, Self::input_ptr(input))) }
    }

    pub fn color_filter(color_filter: &ColorFilter, input: Option<&ImageFilter>) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ImageFilter_ColorFilter(color_filter.0, Self::input_ptr(input))) }
    }

    /// `result = outer(inner(source))`.
    pub fn compose(outer: &ImageFilter, inner: &ImageFilter) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ImageFilter_Compose(outer.0, inner.0)) }
    }

    pub fn matrix_transform(matrix: &Matrix, sampling: SamplingOptions, input: Option<&ImageFilter>) -> Option<Self> {
        let (cubic_b, cubic_c) = sampling.cubic.unwrap_or((0.0, 0.0));
        unsafe {
            Self::from_raw(sys::skialin_bridge_ImageFilter_MatrixTransform(
                &matrix.0,
                sampling.max_aniso,
                sampling.cubic.is_some(),
                cubic_b,
                cubic_c,
                sampling.filter.into(),
                sampling.mipmap.into(),
                Self::input_ptr(input),
            ))
        }
    }

    pub fn dilate(radius_x: f32, radius_y: f32, input: Option<&ImageFilter>) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ImageFilter_Dilate(radius_x, radius_y, Self::input_ptr(input))) }
    }

    pub fn erode(radius_x: f32, radius_y: f32, input: Option<&ImageFilter>) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ImageFilter_Erode(radius_x, radius_y, Self::input_ptr(input))) }
    }
}

impl Drop for ImageFilter {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_ImageFilter_unref(self.0) };
    }
}
