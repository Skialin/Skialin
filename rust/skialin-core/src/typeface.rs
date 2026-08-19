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

    pub fn table_tags(&self) -> Vec<u32> {
        let count = unsafe { sys::SkTypeface_countTables(self.0) };
        let mut tags = vec![0u32; count.max(0) as usize];
        let span = sys::SkSpan { _phantom_0: std::marker::PhantomData, fPtr: tags.as_mut_ptr(), fSize: tags.len() };
        unsafe { sys::SkTypeface_readTableTags(self.0, span) };
        tags
    }

    pub fn table_size(&self, tag: u32) -> usize {
        unsafe { sys::SkTypeface_getTableSize(self.0, tag) }
    }

    pub fn table_data(&self, tag: u32, offset: usize, length: usize) -> Vec<u8> {
        let mut buf = vec![0u8; length];
        let copied = unsafe { sys::SkTypeface_getTableData(self.0, tag, offset, length, buf.as_mut_ptr().cast()) };
        buf.truncate(copied);
        buf
    }
}

impl Drop for Typeface {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Typeface_unref(self.0) };
    }
}
