use crate::{sys, AlphaType, Color, ColorSpace, ColorType, IRect, ISize, ImageInfo};

/// Pairs an [`ImageInfo`] with a pixel buffer and row stride. Mirrors Skia's
/// `SkPixmap`: it never owns the pixel memory, so `addr` must outlive the
/// `Pixmap`. Heap-allocated because `SkPixmap` holds a non-trivial
/// `SkImageInfo` member by value (same by-value ABI hazard as `SkPath`).
pub struct Pixmap(*mut sys::SkPixmap);

impl Pixmap {
    /// # Safety
    /// `addr` must remain valid, and its buffer at least `row_bytes * info.height()`
    /// bytes, for as long as the returned `Pixmap` (and anything cloned from
    /// it via `extract_subset`) is alive.
    pub unsafe fn new(info: &ImageInfo, addr: *const u8, row_bytes: usize) -> Self {
        Pixmap(sys::skialin_bridge_Pixmap_make(info.0, addr.cast(), row_bytes))
    }

    /// No pixels, `kUnknown` color type, zero size. Useful as an
    /// out-parameter target, e.g. for [`crate::Image::peek_pixels`].
    pub fn empty() -> Self {
        Pixmap(unsafe { sys::skialin_bridge_Pixmap_makeEmpty() })
    }

    pub(crate) fn as_raw(&self) -> *mut sys::SkPixmap {
        self.0
    }

    pub fn row_bytes(&self) -> usize {
        unsafe { (*self.0).rowBytes() }
    }

    pub fn addr(&self) -> *const u8 {
        unsafe { (*self.0).addr() as *const u8 }
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

    pub fn is_empty(&self) -> bool {
        unsafe { (*self.0).isEmpty() }
    }

    pub fn color_type(&self) -> ColorType {
        unsafe { (*self.0).colorType() }.into()
    }

    pub fn alpha_type(&self) -> AlphaType {
        unsafe { (*self.0).alphaType() }.into()
    }

    pub fn is_opaque(&self) -> bool {
        unsafe { (*self.0).isOpaque() }
    }

    pub fn color_space(&self) -> Option<ColorSpace> {
        unsafe { ColorSpace::from_raw(sys::skialin_bridge_Pixmap_refColorSpace(self.0)) }
    }

    pub fn row_bytes_as_pixels(&self) -> i32 {
        unsafe { (*self.0).rowBytesAsPixels() }
    }

    pub fn shift_per_pixel(&self) -> i32 {
        unsafe { (*self.0).shiftPerPixel() }
    }

    pub fn compute_byte_size(&self) -> usize {
        unsafe { (*self.0).computeByteSize() }
    }

    /// The full pixel buffer, per [`Pixmap::compute_byte_size`].
    ///
    /// # Safety
    /// The backing memory passed to [`Pixmap::new`] must still be valid.
    pub unsafe fn as_bytes(&self) -> &[u8] {
        std::slice::from_raw_parts(self.addr(), self.compute_byte_size())
    }

    /// Unpremultiplied color at `(x, y)`. Ignores color space; input is not
    /// bounds-checked (matches `SkPixmap::getColor`).
    pub fn get_color(&self, x: i32, y: i32) -> Color {
        unsafe { (*self.0).getColor(x, y) }
    }

    pub fn get_alphaf(&self, x: i32, y: i32) -> f32 {
        unsafe { (*self.0).getAlphaf(x, y) }
    }

    /// The intersection of this pixmap with `area`, sharing the same backing
    /// storage, or `None` if the intersection is empty.
    ///
    /// # Safety
    /// The backing memory passed to the original [`Pixmap::new`] must
    /// outlive the returned subset too.
    pub unsafe fn extract_subset(&self, area: IRect) -> Option<Pixmap> {
        let ptr = sys::skialin_bridge_Pixmap_extractSubset(self.0, area.left, area.top, area.right, area.bottom);
        (!ptr.is_null()).then_some(Pixmap(ptr))
    }
}

impl Drop for Pixmap {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Pixmap_delete(self.0) };
    }
}
