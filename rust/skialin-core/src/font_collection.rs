use crate::{sys, FontMgr};

pub struct FontCollection(pub(crate) *mut sys::skia::textlayout::FontCollection);

impl FontCollection {
    pub fn new() -> Self {
        FontCollection(unsafe { sys::skialin_bridge_FontCollection_new() })
    }

    pub fn set_default_font_manager(&mut self, font_manager: &FontMgr) {
        unsafe { sys::skialin_bridge_FontCollection_setDefaultFontManager(self.0, font_manager.0) };
    }

    pub fn set_asset_font_manager(&mut self, font_manager: &FontMgr) {
        unsafe { sys::skialin_bridge_FontCollection_setAssetFontManager(self.0, font_manager.0) };
    }

    pub fn set_dynamic_font_manager(&mut self, font_manager: &FontMgr) {
        unsafe { sys::skialin_bridge_FontCollection_setDynamicFontManager(self.0, font_manager.0) };
    }

    pub fn set_test_font_manager(&mut self, font_manager: &FontMgr) {
        unsafe { sys::skialin_bridge_FontCollection_setTestFontManager(self.0, font_manager.0) };
    }

    pub fn disable_font_fallback(&mut self) {
        unsafe { sys::skialin_bridge_FontCollection_disableFontFallback(self.0) };
    }

    pub fn enable_font_fallback(&mut self) {
        unsafe { sys::skialin_bridge_FontCollection_enableFontFallback(self.0) };
    }

    pub fn font_fallback_enabled(&mut self) -> bool {
        unsafe { sys::skialin_bridge_FontCollection_fontFallbackEnabled(self.0) }
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
