use crate::{sys, Data, FontStyle};

pub struct Typeface(pub(crate) *mut sys::SkTypeface);

impl Typeface {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::SkTypeface) -> Option<Self> {
        (!ptr.is_null()).then_some(Typeface(ptr))
    }

    pub fn empty() -> Self {
        unsafe { Self::from_raw(sys::skialin_bridge_Typeface_MakeEmpty()).expect("MakeEmpty never returns null") }
    }

    pub fn unique_id(&self) -> u32 {
        unsafe { sys::skialin_bridge_Typeface_uniqueID(self.0) }
    }

    pub fn is_bold(&self) -> bool {
        unsafe { sys::skialin_bridge_Typeface_isBold(self.0) }
    }

    pub fn is_italic(&self) -> bool {
        unsafe { sys::skialin_bridge_Typeface_isItalic(self.0) }
    }

    pub fn is_fixed_pitch(&self) -> bool {
        unsafe { sys::skialin_bridge_Typeface_isFixedPitch(self.0) }
    }

    pub fn count_glyphs(&self) -> i32 {
        unsafe { sys::skialin_bridge_Typeface_countGlyphs(self.0) }
    }

    pub fn units_per_em(&self) -> i32 {
        unsafe { sys::skialin_bridge_Typeface_getUnitsPerEm(self.0) }
    }

    pub fn unichar_to_glyph(&self, unichar: i32) -> u16 {
        unsafe { sys::skialin_bridge_Typeface_unicharToGlyph(self.0, unichar) }
    }

    pub fn font_style(&self) -> FontStyle {
        let (mut weight, mut width, mut slant) = (0, 0, 0);
        unsafe { sys::skialin_bridge_Typeface_fontStyle(self.0, &mut weight, &mut width, &mut slant) };
        FontStyle::new(weight, width, slant.into())
    }

    pub fn family_name(&self) -> String {
        let data = unsafe { Data::from_raw(sys::skialin_bridge_Typeface_familyName(self.0)) }.expect("familyName never returns null");
        String::from_utf8_lossy(data.as_bytes()).into_owned()
    }
}

impl Drop for Typeface {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Typeface_unref(self.0) };
    }
}
