use crate::sys;

/// A custom, caller-implemented draw command. Mirrors Skia's `SkDrawable`.
/// Construction is deliberately not exposed here: the calling side needs a
/// place to route `onDraw`/`onGetBounds` back into (a JVM callback for the
/// `skialin-jni` crate; a plain Rust closure for other native embedders),
/// which only the caller can provide.
pub struct Drawable(*mut sys::SkDrawable);

impl Drawable {
    /// # Safety
    /// `ptr` must be a valid, owned `SkDrawable*` (e.g. from
    /// `skialin_bridge_Drawable_Make`).
    pub unsafe fn from_raw(ptr: *mut sys::SkDrawable) -> Self {
        Drawable(ptr)
    }

    pub fn as_raw(&self) -> *mut sys::SkDrawable {
        self.0
    }

    pub fn make_picture_snapshot(&mut self) -> Option<crate::Picture> {
        let ptr = unsafe { sys::skialin_bridge_Drawable_makePictureSnapshot(self.0) };
        (!ptr.is_null()).then(|| crate::Picture(ptr))
    }

    pub fn bounds(&mut self) -> crate::Rect {
        let mut out = sys::SkRect::default();
        unsafe { sys::skialin_bridge_Drawable_getBounds(self.0, &mut out) };
        out.into()
    }

    pub fn generation_id(&mut self) -> u32 {
        unsafe { sys::skialin_bridge_Drawable_getGenerationID(self.0) }
    }

    pub fn notify_drawing_changed(&mut self) {
        unsafe { sys::skialin_bridge_Drawable_notifyDrawingChanged(self.0) };
    }
}

impl Drop for Drawable {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Drawable_unref(self.0) };
    }
}
