use crate::{sys, Canvas, Rect};
use std::marker::PhantomData;

pub struct PictureRecorder(*mut sys::SkPictureRecorder);

impl PictureRecorder {
    pub fn new() -> Self {
        PictureRecorder(unsafe { sys::skialin_bridge_PictureRecorder_new() })
    }

    pub fn begin_recording(&mut self, bounds: Rect) -> Canvas<'_> {
        let sk_rect: sys::SkRect = bounds.into();
        let ptr = unsafe { sys::skialin_bridge_PictureRecorder_beginRecording(self.0, &sk_rect) };
        Canvas { ptr, _marker: PhantomData }
    }

    pub fn recording_canvas(&mut self) -> Canvas<'_> {
        let ptr = unsafe { sys::skialin_bridge_PictureRecorder_getRecordingCanvas(self.0) };
        Canvas { ptr, _marker: PhantomData }
    }

    pub fn finish_recording_as_picture(&mut self) -> Option<Picture> {
        let ptr = unsafe { sys::skialin_bridge_PictureRecorder_finishRecordingAsPicture(self.0) };
        (!ptr.is_null()).then_some(Picture(ptr))
    }
}

impl Default for PictureRecorder {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for PictureRecorder {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_PictureRecorder_delete(self.0) };
    }
}

pub struct Picture(pub(crate) *mut sys::SkPicture);

impl Picture {
    pub fn playback(&self, canvas: &mut Canvas) {
        unsafe { sys::skialin_bridge_Picture_playback(self.0, canvas.ptr) };
    }

    pub fn cull_rect(&self) -> Rect {
        let mut out = sys::SkRect::default();
        unsafe { sys::skialin_bridge_Picture_cullRect(self.0, &mut out) };
        out.into()
    }

    pub fn unique_id(&self) -> u32 {
        unsafe { sys::skialin_bridge_Picture_uniqueID(self.0) }
    }

    pub fn approximate_op_count(&self, nested: bool) -> i32 {
        unsafe { sys::skialin_bridge_Picture_approximateOpCount(self.0, nested) }
    }
}

impl Drop for Picture {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Picture_unref(self.0) };
    }
}
