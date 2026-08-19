use crate::{sys, AlphaType, Color, ColorSpace, ColorType, IRect, ISize, ImageInfo, SamplingOptions};

pub struct Pixmap(*mut sys::SkPixmap);

impl Pixmap {
    pub unsafe fn new(info: &ImageInfo, addr: *const u8, row_bytes: usize) -> Self {
        Pixmap(sys::skialin_bridge_Pixmap_make(info.0, addr.cast(), row_bytes))
    }

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

    pub unsafe fn as_bytes(&self) -> &[u8] {
        let ptr = self.addr();
        let size = self.compute_byte_size();
        if ptr.is_null() || size == 0 {
            return &[];
        }
        std::slice::from_raw_parts(ptr, size)
    }

    pub fn get_color(&self, x: i32, y: i32) -> Color {
        unsafe { (*self.0).getColor(x, y) }
    }

    pub fn get_alphaf(&self, x: i32, y: i32) -> f32 {
        unsafe { (*self.0).getAlphaf(x, y) }
    }

    pub unsafe fn extract_subset(&self, area: IRect) -> Option<Pixmap> {
        let ptr = sys::skialin_bridge_Pixmap_extractSubset(self.0, area.left, area.top, area.right, area.bottom);
        (!ptr.is_null()).then_some(Pixmap(ptr))
    }

    pub fn read_pixels(&self, dst: &mut Pixmap, src_x: i32, src_y: i32) -> bool {
        let dst_info = ImageInfo::new(dst.width(), dst.height(), dst.color_type(), dst.alpha_type());
        unsafe { sys::SkPixmap_readPixels1(self.0, dst_info.0, dst.addr().cast_mut().cast(), dst.row_bytes(), src_x, src_y) }
    }

    pub fn scale_pixels(&self, dst: &mut Pixmap, sampling: SamplingOptions) -> bool {
        let sk_sampling = crate::canvas::to_sk_sampling(sampling);
        unsafe { sys::SkPixmap_scalePixels(self.0, dst.0, &sk_sampling) }
    }
}

impl Drop for Pixmap {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Pixmap_delete(self.0) };
    }
}
