use std::marker::PhantomData;

use crate::{sys, Canvas, Data, Rect};

/// Draw flags for [`SVGCanvas::new`]. Matches `SkSVGCanvas::Flags`.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct SVGCanvasFlags {
    /// Emit text as `<path>`s instead of `<text>`.
    pub convert_text_to_paths: bool,
    /// Suppress newlines/indentation in the output XML.
    pub no_pretty_xml: bool,
    /// Encode path data with relative commands.
    pub relative_path_encoding: bool,
}

impl From<SVGCanvasFlags> for u32 {
    fn from(flags: SVGCanvasFlags) -> Self {
        (flags.convert_text_to_paths as u32) | ((flags.no_pretty_xml as u32) << 1) | ((flags.relative_path_encoding as u32) << 2)
    }
}

/// Records `Canvas` draw calls as SVG XML. Mirrors Skia's `SkSVGCanvas`.
pub struct SVGCanvas(*mut sys::SkialinSvgCanvas);

impl SVGCanvas {
    /// `bounds` becomes the root `<svg>` element's `viewBox`.
    pub fn new(bounds: Rect, flags: SVGCanvasFlags) -> Self {
        let sk_bounds: sys::SkRect = bounds.into();
        SVGCanvas(unsafe { sys::skialin_bridge_SVGCanvas_Make(&sk_bounds, flags.into()) })
    }

    pub fn canvas(&mut self) -> Canvas<'_> {
        let ptr = unsafe { sys::skialin_bridge_SVGCanvas_getCanvas(self.0) };
        Canvas { ptr, _marker: PhantomData }
    }

    /// Flushes and returns the recorded SVG XML, consuming the canvas.
    pub fn finish(mut self) -> Data {
        let ptr = unsafe { sys::skialin_bridge_SVGCanvas_finish(self.0) };
        self.0 = std::ptr::null_mut();
        unsafe { Data::from_raw(ptr) }.expect("SVGCanvas_finish never returns null")
    }
}

impl Drop for SVGCanvas {
    fn drop(&mut self) {
        if !self.0.is_null() {
            let data = unsafe { sys::skialin_bridge_SVGCanvas_finish(self.0) };
            unsafe { sys::skialin_bridge_Data_unref(data) };
        }
    }
}
