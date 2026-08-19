use crate::{sys, ImageInfo};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FrameInfo {
    pub duration_ms: i32,
    /// The earliest frame this one can be blended with, if any.
    pub required_frame: Option<i32>,
    pub fully_received: bool,
}

/// Decodes an image, exposing multi-frame (animated GIF/WEBP) introspection
/// and explicit per-frame decoding beyond [`crate::Image::decode`]'s
/// implicit first-frame-only decode. Mirrors Skia's `SkCodec`.
pub struct Codec(*mut sys::SkCodec);

impl Codec {
    /// `None` if `bytes` isn't a recognized image format.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let ptr = unsafe { sys::skialin_bridge_Codec_MakeFromData(bytes.as_ptr(), bytes.len()) };
        (!ptr.is_null()).then_some(Codec(ptr))
    }

    pub fn dimensions(&self) -> (i32, i32) {
        let mut width = 0;
        let mut height = 0;
        unsafe { sys::skialin_bridge_Codec_dimensions(self.0, &mut width, &mut height) };
        (width, height)
    }

    /// The container format's `SkEncodedImageFormat` ordinal (0 = BMP, 1 =
    /// GIF, 2 = ICO, 3 = JPEG, 4 = PNG, 5 = WBMP, 6 = WEBP, 7 = PKM, 8 = KTX,
    /// 9 = ASTC, 10 = DNG, 11 = HEIF, 12 = AVIF, 13 = JPEGXL).
    pub fn encoded_format(&self) -> i32 {
        unsafe { sys::skialin_bridge_Codec_getEncodedFormat(self.0) }
    }

    /// 1 for a static image; the number of frames for an animated one.
    pub fn frame_count(&self) -> i32 {
        unsafe { sys::skialin_bridge_Codec_getFrameCount(self.0) }
    }

    pub fn frame_info(&self, index: i32) -> Option<FrameInfo> {
        let mut duration_ms = 0;
        let mut required_frame = -1;
        let mut fully_received = false;
        let ok = unsafe { sys::skialin_bridge_Codec_getFrameInfo(self.0, index, &mut duration_ms, &mut required_frame, &mut fully_received) };
        ok.then_some(FrameInfo { duration_ms, required_frame: (required_frame >= 0).then_some(required_frame), fully_received })
    }

    /// Decodes `frame_index` (`0` for static images) into `dst_pixels`, a
    /// buffer of at least `dst_row_bytes * height` bytes matching
    /// `dst_info`. Returns `true` on success.
    ///
    /// # Safety
    /// `dst_pixels` must be valid for `dst_row_bytes * height` writable bytes.
    pub unsafe fn get_pixels(&mut self, dst_info: &ImageInfo, dst_pixels: *mut u8, dst_row_bytes: usize, frame_index: i32) -> bool {
        sys::skialin_bridge_Codec_getPixels(self.0, dst_info.0, dst_pixels.cast(), dst_row_bytes, frame_index) == 0
    }
}

impl Drop for Codec {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Codec_delete(self.0) };
    }
}
