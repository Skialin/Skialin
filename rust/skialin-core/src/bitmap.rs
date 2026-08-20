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

    /// Sets this bitmap's [ImageInfo] and pixel storage to a copy of
    /// `pixels`, replacing whatever pixel storage it had before. The copy is
    /// heap-allocated on the Rust side and freed via Skia's releaseProc once
    /// the bitmap no longer needs it, so `pixels` doesn't need to outlive
    /// this call.
    pub fn install_pixels(&mut self, info: &ImageInfo, pixels: &[u8], row_bytes: usize) -> bool {
        let owned: Box<[u8]> = pixels.into();
        let ptr = owned.as_ptr() as *mut std::ffi::c_void;
        let context = Box::into_raw(Box::new(owned)) as *mut std::ffi::c_void;
        // installPixels invokes releaseProc itself on every path (failure,
        // null pixels, or success), so it always reclaims `context`.
        unsafe { self.0.installPixels(info.0, ptr, row_bytes, Some(release_install_pixels), context) }
    }

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

    pub fn extract_subset(&self, dst: &mut Bitmap, subset: crate::IRect) -> bool {
        let sk_subset: sys::SkIRect = subset.into();
        unsafe { self.0.extractSubset(dst.as_raw_mut(), &sk_subset) }
    }

    pub fn extract_alpha(&self, dst: &mut Bitmap) -> bool {
        unsafe { self.0.extractAlpha2(dst.as_raw_mut(), std::ptr::null(), std::ptr::null_mut(), std::ptr::null_mut()) }
    }

    pub fn notify_pixels_changed(&self) {
        unsafe { self.0.notifyPixelsChanged() };
    }
}

unsafe extern "C" fn release_install_pixels(_addr: *mut std::ffi::c_void, context: *mut std::ffi::c_void) {
    drop(Box::from_raw(context as *mut Box<[u8]>));
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
