use crate::{sys, Canvas};

/// A parsed SVG document. Mirrors Skia's `SkSVGDOM`.
pub struct SVGDOM(*mut sys::SkSVGDOM);

impl SVGDOM {
    /// Parses `bytes` as SVG XML. `None` if it doesn't parse.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let ptr = unsafe { sys::skialin_bridge_SVGDOM_MakeFromStream(bytes.as_ptr(), bytes.len()) };
        (!ptr.is_null()).then_some(SVGDOM(ptr))
    }

    /// Sets the viewport used to resolve the root's width/height when
    /// they're specified in relative units.
    pub fn set_container_size(&mut self, width: f32, height: f32) {
        unsafe { sys::skialin_bridge_SVGDOM_setContainerSize(self.0, width, height) };
    }

    pub fn container_size(&self) -> (f32, f32) {
        let mut width = 0.0;
        let mut height = 0.0;
        unsafe { sys::skialin_bridge_SVGDOM_getContainerSize(self.0, &mut width, &mut height) };
        (width, height)
    }

    pub fn render(&self, canvas: &mut Canvas) {
        unsafe { sys::skialin_bridge_SVGDOM_render(self.0, canvas.as_raw()) };
    }
}

impl Drop for SVGDOM {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_SVGDOM_unref(self.0) };
    }
}
