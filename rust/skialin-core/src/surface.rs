use crate::{sys, Canvas, Image, ImageInfo};
use std::marker::PhantomData;

pub struct Surface(*mut sys::SkSurface);

impl Surface {
    pub fn new_raster_n32_premul(width: i32, height: i32) -> Option<Self> {
        let ptr = unsafe { sys::skialin_bridge_Surface_MakeRasterN32Premul(width, height) };
        (!ptr.is_null()).then_some(Surface(ptr))
    }

    pub fn new_raster(info: &ImageInfo) -> Option<Self> {
        let ptr = unsafe { sys::skialin_bridge_Surface_MakeRaster(info.0) };
        (!ptr.is_null()).then_some(Surface(ptr))
    }

    pub fn canvas(&mut self) -> Canvas<'_> {
        let ptr = unsafe { sys::skialin_bridge_Surface_getCanvas(self.0) };
        Canvas { ptr, _marker: PhantomData }
    }

    pub fn image_snapshot(&mut self) -> Option<Image> {
        let ptr = unsafe { sys::skialin_bridge_Surface_makeImageSnapshot(self.0) };
        (!ptr.is_null()).then(|| Image(ptr))
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Surface_unref(self.0) };
    }
}
