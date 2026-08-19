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

    pub fn drop_shadow_only(dx: f32, dy: f32, sigma_x: f32, sigma_y: f32, color: Color, input: Option<&ImageFilter>) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ImageFilter_DropShadowOnly(dx, dy, sigma_x, sigma_y, color, Self::input_ptr(input))) }
    }

    pub fn offset(dx: f32, dy: f32, input: Option<&ImageFilter>) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ImageFilter_Offset(dx, dy, Self::input_ptr(input))) }
    }

    pub fn color_filter(color_filter: &ColorFilter, input: Option<&ImageFilter>) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ImageFilter_ColorFilter(color_filter.0, Self::input_ptr(input))) }
    }

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

    pub fn blend(mode: crate::BlendMode, background: Option<&ImageFilter>, foreground: Option<&ImageFilter>) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ImageFilter_Blend(mode.into(), Self::input_ptr(background), Self::input_ptr(foreground))) }
    }

    pub fn merge(first: Option<&ImageFilter>, second: Option<&ImageFilter>) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ImageFilter_Merge(Self::input_ptr(first), Self::input_ptr(second))) }
    }

    pub fn shader(shader: &crate::Shader) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ImageFilter_Shader(shader.0)) }
    }

    pub fn tile(src: crate::Rect, dst: crate::Rect, input: Option<&ImageFilter>) -> Option<Self> {
        let sk_src: sys::SkRect = src.into();
        let sk_dst: sys::SkRect = dst.into();
        unsafe { Self::from_raw(sys::skialin_bridge_ImageFilter_Tile(&sk_src, &sk_dst, Self::input_ptr(input))) }
    }
}

impl Drop for ImageFilter {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_ImageFilter_unref(self.0) };
    }
}
