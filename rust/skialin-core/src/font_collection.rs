use crate::{sys, FontMgr};

pub struct FontCollection(pub(crate) *mut sys::skia::textlayout::FontCollection);

impl FontCollection {
    pub fn new() -> Self {
        FontCollection(unsafe { sys::skialin_bridge_FontCollection_new() })
    }

    /// The minimum needed to get real glyphs out of a laid-out paragraph;
    /// usually [`FontMgr::system`].
    pub fn set_default_font_manager(&mut self, font_manager: &FontMgr) {
        unsafe { sys::skialin_bridge_FontCollection_setDefaultFontManager(self.0, font_manager.0) };
    }
}

impl Default for FontCollection {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for FontCollection {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_FontCollection_unref(self.0) };
    }
}
