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
}

impl Drop for Drawable {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Drawable_unref(self.0) };
    }
}
