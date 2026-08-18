use crate::{sys, AlphaType, ColorSpace, ColorType, IRect, ISize};

pub struct ImageInfo(pub(crate) *mut sys::SkImageInfo);

impl ImageInfo {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::SkImageInfo) -> Self {
        ImageInfo(ptr)
    }

    pub fn new(width: i32, height: i32, color_type: ColorType, alpha_type: AlphaType) -> Self {
        Self::with_color_space(width, height, color_type, alpha_type, None)
    }

    pub fn with_color_space(width: i32, height: i32, color_type: ColorType, alpha_type: AlphaType, color_space: Option<&ColorSpace>) -> Self {
        let cs_ptr = color_space.map_or(std::ptr::null_mut(), |cs| cs.0);
        ImageInfo(unsafe { sys::skialin_bridge_ImageInfo_make(width, height, color_type.into(), alpha_type.into(), cs_ptr) })
    }

    pub fn n32_premul(width: i32, height: i32) -> Self {
        Self::new(width, height, ColorType::N32, AlphaType::Premul)
    }

    pub fn width(&self) -> i32 {
        unsafe { (*self.0).width() }
    }

    pub fn height(&self) -> i32 {
        unsafe { (*self.0).height() }
    }

    pub fn dimensions(&self) -> ISize {
        ISize::new(self.width(), self.height())
    }

    pub fn bounds(&self) -> IRect {
        IRect::from_wh(self.width(), self.height())
    }

    pub fn color_type(&self) -> ColorType {
        unsafe { (*self.0).colorType() }.into()
    }

    pub fn alpha_type(&self) -> AlphaType {
        unsafe { (*self.0).alphaType() }.into()
    }

    /// Borrowed; valid only for as long as this `ImageInfo` is alive.
    pub fn color_space(&self) -> Option<ColorSpace> {
        unsafe { ColorSpace::from_raw(sys::skialin_bridge_ImageInfo_refColorSpace(self.0)) }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { (*self.0).isEmpty() }
    }

    pub fn is_opaque(&self) -> bool {
        unsafe { (*self.0).isOpaque() }
    }

    pub fn gamma_close_to_srgb(&self) -> bool {
        unsafe { (*self.0).gammaCloseToSRGB() }
    }

    pub fn bytes_per_pixel(&self) -> i32 {
        unsafe { (*self.0).bytesPerPixel() }
    }

    pub fn shift_per_pixel(&self) -> i32 {
        unsafe { (*self.0).shiftPerPixel() }
    }

    pub fn min_row_bytes(&self) -> usize {
        unsafe { (*self.0).minRowBytes() }
    }

    pub fn compute_min_byte_size(&self) -> usize {
        unsafe { (*self.0).computeMinByteSize() }
    }

    pub fn compute_byte_size(&self, row_bytes: usize) -> usize {
        unsafe { (*self.0).computeByteSize(row_bytes) }
    }

    pub fn valid_row_bytes(&self, row_bytes: usize) -> bool {
        unsafe { (*self.0).validRowBytes(row_bytes) }
    }

    pub fn with_wh(&self, width: i32, height: i32) -> Self {
        ImageInfo(unsafe { sys::skialin_bridge_ImageInfo_makeWH(self.0, width, height) })
    }

    pub fn with_color_type(&self, color_type: ColorType) -> Self {
        ImageInfo(unsafe { sys::skialin_bridge_ImageInfo_makeColorType(self.0, color_type.into()) })
    }

    pub fn with_alpha_type(&self, alpha_type: AlphaType) -> Self {
        ImageInfo(unsafe { sys::skialin_bridge_ImageInfo_makeAlphaType(self.0, alpha_type.into()) })
    }

    pub fn with_color_space_opt(&self, color_space: Option<&ColorSpace>) -> Self {
        let cs_ptr = color_space.map_or(std::ptr::null_mut(), |cs| cs.0);
        ImageInfo(unsafe { sys::skialin_bridge_ImageInfo_makeColorSpace(self.0, cs_ptr) })
    }

    pub fn equals(&self, other: &ImageInfo) -> bool {
        unsafe { sys::skialin_bridge_ImageInfo_equals(self.0, other.0) }
    }
}

impl Clone for ImageInfo {
    fn clone(&self) -> Self {
        self.with_wh(self.width(), self.height())
    }
}

impl Drop for ImageInfo {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_ImageInfo_delete(self.0) };
    }
}
