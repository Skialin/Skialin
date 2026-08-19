use crate::{sys, Canvas, Rect};

/// A Lottie/Bodymovin JSON animation player. Mirrors Skia's
/// `skottie::Animation`.
pub struct SkottieAnimation(*mut sys::skottie::Animation);

impl SkottieAnimation {
    /// `None` if `data` doesn't parse as a valid animation.
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        let ptr = unsafe { sys::skialin_bridge_SkottieAnimation_Make(data.as_ptr().cast(), data.len()) };
        (!ptr.is_null()).then_some(SkottieAnimation(ptr))
    }

    /// Renders at `dst`, or at the animation's intrinsic size at the origin
    /// when `dst` is `None`.
    pub fn render(&self, canvas: &mut Canvas, dst: Option<Rect>) {
        let sk_dst: Option<sys::SkRect> = dst.map(Into::into);
        let dst_ptr = sk_dst.as_ref().map_or(std::ptr::null(), |r| r as *const sys::SkRect);
        unsafe { sys::skialin_bridge_SkottieAnimation_render(self.0, canvas.as_raw(), dst_ptr) };
    }

    /// `t` is normalized progress in `[0, 1]`.
    pub fn seek(&mut self, t: f32) {
        unsafe { sys::skialin_bridge_SkottieAnimation_seek(self.0, t) };
    }

    pub fn seek_frame(&mut self, frame: f64) {
        unsafe { sys::skialin_bridge_SkottieAnimation_seekFrame(self.0, frame) };
    }

    /// Total duration in seconds.
    pub fn duration(&self) -> f64 {
        unsafe { sys::skialin_bridge_SkottieAnimation_duration(self.0) }
    }

    pub fn fps(&self) -> f64 {
        unsafe { sys::skialin_bridge_SkottieAnimation_fps(self.0) }
    }

    pub fn size(&self) -> (f32, f32) {
        let mut width = 0.0;
        let mut height = 0.0;
        unsafe { sys::skialin_bridge_SkottieAnimation_size(self.0, &mut width, &mut height) };
        (width, height)
    }
}

impl Drop for SkottieAnimation {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_SkottieAnimation_unref(self.0) };
    }
}
