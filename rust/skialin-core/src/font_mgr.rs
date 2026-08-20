use std::ffi::CString;

use crate::{sys, Data, FontStyle, Typeface};

pub struct FontMgr(pub(crate) *mut sys::SkFontMgr);

impl FontMgr {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::SkFontMgr) -> Option<Self> {
        (!ptr.is_null()).then_some(FontMgr(ptr))
    }

    /// The platform's default font manager (DirectWrite on Windows, CoreText on macOS, FontConfig on Linux).
    pub fn system() -> Self {
        unsafe { Self::from_raw(sys::skialin_bridge_FontMgr_RefSystem()).expect("RefSystem never returns null") }
    }

    pub fn empty() -> Self {
        unsafe { Self::from_raw(sys::skialin_bridge_FontMgr_RefEmpty()).expect("RefEmpty never returns null") }
    }

    pub fn count_families(&self) -> i32 {
        unsafe { sys::skialin_bridge_FontMgr_countFamilies(self.0) }
    }

    pub fn family_name(&self, index: i32) -> String {
        let data = unsafe { Data::from_raw(sys::skialin_bridge_FontMgr_familyName(self.0, index)) }.expect("familyName never returns null");
        String::from_utf8_lossy(data.as_bytes()).into_owned()
    }

    /// `family_name` of `None` requests the default system family, which
    /// most systems don't have, so it will often fall through to `None` here.
    pub fn match_family_style(&self, family_name: Option<&str>, style: FontStyle) -> Option<Typeface> {
        let c_name = family_name.map(|s| CString::new(s).expect("family_name must not contain a NUL byte"));
        let name_ptr = c_name.as_ref().map_or(std::ptr::null(), |s| s.as_ptr());
        unsafe { Typeface::from_raw(sys::skialin_bridge_FontMgr_matchFamilyStyle(self.0, name_ptr, style.weight, style.width, style.slant.into())) }
    }

    /// `data` is ref'd by the bridge, not consumed: it stays independently
    /// valid and closeable afterward.
    pub fn make_from_data(&self, data: &Data, ttc_index: i32) -> Option<Typeface> {
        unsafe { Typeface::from_raw(sys::skialin_bridge_FontMgr_makeFromData(self.0, data.0, ttc_index)) }
    }

    pub fn make_from_file(&self, path: &str, ttc_index: i32) -> Option<Typeface> {
        let c_path = CString::new(path).expect("path must not contain a NUL byte");
        unsafe { Typeface::from_raw(sys::skialin_bridge_FontMgr_makeFromFile(self.0, c_path.as_ptr(), ttc_index)) }
    }
}

impl Drop for FontMgr {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_FontMgr_unref(self.0) };
    }
}

/// A concrete `SkFontMgr` (`skia::textlayout::TypefaceFontProvider`) that resolves family names
/// to in-memory-registered typefaces. Register typefaces with [`Self::register_typeface`], then
/// hand it to [`crate::FontCollection::set_asset_font_manager`] (or one of the other
/// `set_*_font_manager` methods) so paragraph shaping/fallback can resolve names to these
/// typefaces - the standard way to make custom/embedded fonts participate in name-based
/// fallback resolution during layout, the same as system fonts.
pub struct TypefaceFontProvider(*mut sys::SkFontMgr);

impl TypefaceFontProvider {
    pub fn new() -> Self {
        TypefaceFontProvider(unsafe { sys::skialin_bridge_TypefaceFontProvider_new() })
    }

    /// Registers `typeface` under its own family name. `typeface` is ref'd, not consumed: it
    /// stays independently valid and closeable afterward. Returns 1 on success, 0 if the
    /// typeface has no family name.
    pub fn register_typeface(&mut self, typeface: &Typeface) -> usize {
        unsafe { sys::skialin_bridge_TypefaceFontProvider_registerTypeface(self.0, typeface.0) }
    }

    /// Registers `typeface` under `alias` instead of its own family name - useful for giving a
    /// `LoadedFont`'s synthetic identity a resolvable family name.
    pub fn register_typeface_with_alias(&mut self, typeface: &Typeface, alias: &str) -> usize {
        unsafe { sys::skialin_bridge_TypefaceFontProvider_registerTypefaceAlias(self.0, typeface.0, alias.as_ptr().cast(), alias.len()) }
    }

    pub(crate) fn as_ptr(&self) -> *mut sys::SkFontMgr {
        self.0
    }
}

impl Default for TypefaceFontProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for TypefaceFontProvider {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_FontMgr_unref(self.0) };
    }
}
