use crate::{sys, Color, Image, ImageInfo};

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

impl From<sys::SkAlphaType> for AlphaType {
    fn from(at: sys::SkAlphaType) -> Self {
        match at as u32 {
            1 => AlphaType::Opaque,
            2 => AlphaType::Premul,
            3 => AlphaType::Unpremul,
            _ => AlphaType::Unknown,
        }
    }
}

pub struct Bitmap(Box<sys::SkBitmap>);

impl Bitmap {
    pub fn new() -> Self {
        Bitmap(crate::support::new_boxed(sys::SkBitmap_SkBitmap))
    }

    pub fn alloc_pixels(&mut self, info: &ImageInfo) {
        unsafe { self.0.allocPixels1(info.0) };
    }

    pub(crate) fn as_raw_mut(&mut self) -> *mut sys::SkBitmap {
        &mut *self.0
    }

    pub(crate) fn as_raw(&self) -> *const sys::SkBitmap {
        &*self.0
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
