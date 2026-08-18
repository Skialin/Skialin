use crate::{sys, Color, Image};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorType {
    Rgba8888,
    Bgra8888,
    Alpha8,
    Gray8,
}

impl From<ColorType> for sys::SkColorType {
    fn from(ct: ColorType) -> Self {
        match ct {
            ColorType::Rgba8888 => sys::SkColorType_kRGBA_8888_SkColorType,
            ColorType::Bgra8888 => sys::SkColorType_kBGRA_8888_SkColorType,
            ColorType::Alpha8 => sys::SkColorType_kAlpha_8_SkColorType,
            ColorType::Gray8 => sys::SkColorType_kGray_8_SkColorType,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AlphaType {
    Unknown,
    Opaque,
    Premul,
    Unpremul,
}

impl From<AlphaType> for sys::SkAlphaType {
    fn from(at: AlphaType) -> Self {
        match at {
            AlphaType::Unknown => sys::SkAlphaType_kUnknown_SkAlphaType,
            AlphaType::Opaque => sys::SkAlphaType_kOpaque_SkAlphaType,
            AlphaType::Premul => sys::SkAlphaType_kPremul_SkAlphaType,
            AlphaType::Unpremul => sys::SkAlphaType_kUnpremul_SkAlphaType,
        }
    }
}

pub struct Bitmap(Box<sys::SkBitmap>);

impl Bitmap {
    pub fn new() -> Self {
        Bitmap(crate::support::new_boxed(sys::SkBitmap_SkBitmap))
    }

    pub fn alloc_pixels(&mut self, width: i32, height: i32, color_type: ColorType, alpha_type: AlphaType) {
        let info = unsafe { sys::SkImageInfo_Make(width, height, color_type.into(), alpha_type.into()) };
        unsafe { self.0.allocPixels1(&info) };
    }

    pub fn width(&self) -> i32 {
        unsafe { self.0.width() }
    }

    pub fn height(&self) -> i32 {
        unsafe { self.0.height() }
    }

    pub fn row_bytes(&self) -> usize {
        unsafe { self.0.rowBytes() }
    }

    pub fn erase_color(&mut self, color: Color) {
        let (a, r, g, b) = (
            ((color >> 24) & 0xff) as u8,
            ((color >> 16) & 0xff) as u8,
            ((color >> 8) & 0xff) as u8,
            (color & 0xff) as u8,
        );
        unsafe { self.0.eraseARGB(a as u32, r as u32, g as u32, b as u32) };
    }

    /// Raw view of the pixel buffer, valid until the next call that
    /// reallocates pixels (e.g. `alloc_pixels`).
    pub fn pixels(&mut self) -> &mut [u8] {
        let len = self.row_bytes() * self.height().max(0) as usize;
        let ptr = unsafe { self.0.getPixels() } as *mut u8;
        if ptr.is_null() || len == 0 {
            return &mut [];
        }
        unsafe { std::slice::from_raw_parts_mut(ptr, len) }
    }

    pub fn as_image(&self) -> Option<Image> {
        let ptr = unsafe { sys::skialin_bridge_Bitmap_asImage(&*self.0) };
        (!ptr.is_null()).then_some(Image(ptr))
    }
}

impl Default for Bitmap {
    fn default() -> Self {
        Bitmap::new()
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        unsafe { self.0.destruct() };
    }
}
