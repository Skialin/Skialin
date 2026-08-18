use crate::{sys, Typeface};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edging {
    Alias,
    AntiAlias,
    SubpixelAntiAlias,
}

impl From<Edging> for sys::SkFont_Edging {
    fn from(edging: Edging) -> Self {
        (match edging {
            Edging::Alias => sys::SkFont_Edging_kAlias,
            Edging::AntiAlias => sys::SkFont_Edging_kAntiAlias,
            Edging::SubpixelAntiAlias => sys::SkFont_Edging_kSubpixelAntiAlias,
        }) as sys::SkFont_Edging
    }
}

impl From<sys::SkFont_Edging> for Edging {
    fn from(edging: sys::SkFont_Edging) -> Self {
        match edging {
            sys::SkFont_Edging_kAntiAlias => Edging::AntiAlias,
            sys::SkFont_Edging_kSubpixelAntiAlias => Edging::SubpixelAntiAlias,
            _ => Edging::Alias,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hinting {
    None,
    Slight,
    Normal,
    Full,
}

impl From<Hinting> for sys::SkFontHinting {
    fn from(hinting: Hinting) -> Self {
        (match hinting {
            Hinting::None => sys::SkFontHinting_kNone,
            Hinting::Slight => sys::SkFontHinting_kSlight,
            Hinting::Normal => sys::SkFontHinting_kNormal,
            Hinting::Full => sys::SkFontHinting_kFull,
        }) as sys::SkFontHinting
    }
}

impl From<sys::SkFontHinting> for Hinting {
    fn from(hinting: sys::SkFontHinting) -> Self {
        match hinting {
            sys::SkFontHinting_kSlight => Hinting::Slight,
            sys::SkFontHinting_kNormal => Hinting::Normal,
            sys::SkFontHinting_kFull => Hinting::Full,
            _ => Hinting::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEncoding {
    Utf8,
    Utf16,
    Utf32,
    GlyphId,
}

impl From<TextEncoding> for sys::SkTextEncoding {
    fn from(encoding: TextEncoding) -> Self {
        (match encoding {
            TextEncoding::Utf8 => sys::SkTextEncoding_kUTF8,
            TextEncoding::Utf16 => sys::SkTextEncoding_kUTF16,
            TextEncoding::Utf32 => sys::SkTextEncoding_kUTF32,
            TextEncoding::GlyphId => sys::SkTextEncoding_kGlyphID,
        }) as sys::SkTextEncoding
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FontMetrics {
    pub top: f32,
    pub ascent: f32,
    pub descent: f32,
    pub bottom: f32,
    pub leading: f32,
    pub avg_char_width: f32,
    pub max_char_width: f32,
    pub x_min: f32,
    pub x_max: f32,
    pub x_height: f32,
    pub cap_height: f32,
}

impl From<sys::SkFontMetrics> for FontMetrics {
    fn from(m: sys::SkFontMetrics) -> Self {
        FontMetrics {
            top: m.fTop,
            ascent: m.fAscent,
            descent: m.fDescent,
            bottom: m.fBottom,
            leading: m.fLeading,
            avg_char_width: m.fAvgCharWidth,
            max_char_width: m.fMaxCharWidth,
            x_min: m.fXMin,
            x_max: m.fXMax,
            x_height: m.fXHeight,
            cap_height: m.fCapHeight,
        }
    }
}

/// `SkSpan<T>` is a plain non-owning {ptr, len} view; the bridge's C++ side
/// treats these spans as read-only despite bindgen collapsing `SkSpan<const
/// T>` and `SkSpan<T>` to the same generic Rust type.
fn span<T>(slice: &[T]) -> sys::SkSpan<T> {
    sys::SkSpan { _phantom_0: std::marker::PhantomData, fPtr: slice.as_ptr().cast_mut(), fSize: slice.len() }
}

fn span_mut<T>(slice: &mut [T]) -> sys::SkSpan<T> {
    sys::SkSpan { _phantom_0: std::marker::PhantomData, fPtr: slice.as_mut_ptr(), fSize: slice.len() }
}

pub struct Font(pub(crate) *mut sys::SkFont);

impl Font {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::SkFont) -> Option<Self> {
        (!ptr.is_null()).then_some(Font(ptr))
    }

    pub fn new() -> Self {
        unsafe { Self::from_raw(sys::skialin_bridge_Font_MakeDefault()).expect("MakeDefault never returns null") }
    }

    pub fn from_typeface(typeface: &Typeface, size: f32) -> Self {
        unsafe { Self::from_raw(sys::skialin_bridge_Font_MakeWithTypeface(typeface.0, size)).expect("MakeWithTypeface never returns null") }
    }

    pub fn typeface(&self) -> Option<Typeface> {
        unsafe { Typeface::from_raw(sys::skialin_bridge_Font_refTypeface(self.0)) }
    }

    pub fn set_typeface(&mut self, typeface: Option<&Typeface>) {
        let ptr = typeface.map_or(std::ptr::null_mut(), |t| t.0);
        unsafe { sys::skialin_bridge_Font_setTypeface(self.0, ptr) };
    }

    pub fn size(&self) -> f32 {
        unsafe { sys::SkFont_getSize(self.0) }
    }

    pub fn set_size(&mut self, size: f32) {
        unsafe { sys::SkFont_setSize(self.0, size) };
    }

    pub fn scale_x(&self) -> f32 {
        unsafe { sys::SkFont_getScaleX(self.0) }
    }

    pub fn set_scale_x(&mut self, scale_x: f32) {
        unsafe { sys::SkFont_setScaleX(self.0, scale_x) };
    }

    pub fn skew_x(&self) -> f32 {
        unsafe { sys::SkFont_getSkewX(self.0) }
    }

    pub fn set_skew_x(&mut self, skew_x: f32) {
        unsafe { sys::SkFont_setSkewX(self.0, skew_x) };
    }

    pub fn edging(&self) -> Edging {
        unsafe { sys::SkFont_getEdging(self.0) }.into()
    }

    pub fn set_edging(&mut self, edging: Edging) {
        unsafe { sys::SkFont_setEdging(self.0, edging.into()) };
    }

    pub fn hinting(&self) -> Hinting {
        unsafe { sys::SkFont_getHinting(self.0) }.into()
    }

    pub fn set_hinting(&mut self, hinting: Hinting) {
        unsafe { sys::SkFont_setHinting(self.0, hinting.into()) };
    }

    pub fn is_subpixel(&self) -> bool {
        unsafe { sys::SkFont_isSubpixel(self.0) }
    }

    pub fn set_subpixel(&mut self, subpixel: bool) {
        unsafe { sys::SkFont_setSubpixel(self.0, subpixel) };
    }

    pub fn is_embolden(&self) -> bool {
        unsafe { sys::SkFont_isEmbolden(self.0) }
    }

    pub fn set_embolden(&mut self, embolden: bool) {
        unsafe { sys::SkFont_setEmbolden(self.0, embolden) };
    }

    pub fn is_linear_metrics(&self) -> bool {
        unsafe { sys::SkFont_isLinearMetrics(self.0) }
    }

    pub fn set_linear_metrics(&mut self, linear_metrics: bool) {
        unsafe { sys::SkFont_setLinearMetrics(self.0, linear_metrics) };
    }

    pub fn is_force_auto_hinting(&self) -> bool {
        unsafe { sys::SkFont_isForceAutoHinting(self.0) }
    }

    pub fn set_force_auto_hinting(&mut self, force_auto_hinting: bool) {
        unsafe { sys::SkFont_setForceAutoHinting(self.0, force_auto_hinting) };
    }

    pub fn is_embedded_bitmaps(&self) -> bool {
        unsafe { sys::SkFont_isEmbeddedBitmaps(self.0) }
    }

    pub fn set_embedded_bitmaps(&mut self, embedded_bitmaps: bool) {
        unsafe { sys::SkFont_setEmbeddedBitmaps(self.0, embedded_bitmaps) };
    }

    pub fn is_baseline_snap(&self) -> bool {
        unsafe { sys::SkFont_isBaselineSnap(self.0) }
    }

    pub fn set_baseline_snap(&mut self, baseline_snap: bool) {
        unsafe { sys::SkFont_setBaselineSnap(self.0, baseline_snap) };
    }

    pub fn unichar_to_glyph(&self, unichar: i32) -> u16 {
        unsafe { sys::SkFont_unicharToGlyph(self.0, unichar) }
    }

    /// Converts UTF-8 text into glyph indices, using this font's typeface's
    /// default character-to-glyph mapping.
    pub fn text_to_glyphs(&self, text: &str) -> Vec<u16> {
        let count = unsafe { sys::SkFont_countText(self.0, text.as_ptr().cast(), text.len(), sys::SkTextEncoding_kUTF8 as sys::SkTextEncoding) };
        let mut glyphs = vec![0u16; count];
        unsafe { sys::SkFont_textToGlyphs(self.0, text.as_ptr().cast(), text.len(), sys::SkTextEncoding_kUTF8 as sys::SkTextEncoding, span_mut(&mut glyphs)) };
        glyphs
    }

    /// The advance width of `text`, encoded as UTF-8.
    pub fn measure_text(&self, text: &str) -> f32 {
        unsafe { sys::SkFont_measureText(self.0, text.as_ptr().cast(), text.len(), sys::SkTextEncoding_kUTF8 as sys::SkTextEncoding, std::ptr::null_mut()) }
    }

    /// The advance width for each glyph in `glyphs`.
    pub fn widths(&self, glyphs: &[u16]) -> Vec<f32> {
        let mut widths = vec![0f32; glyphs.len()];
        unsafe { sys::SkFont_getWidths(self.0, span(glyphs), span_mut(&mut widths)) };
        widths
    }

    pub fn metrics(&self) -> FontMetrics {
        let mut metrics = sys::SkFontMetrics::default();
        unsafe { sys::SkFont_getMetrics(self.0, &mut metrics) };
        metrics.into()
    }

    /// The recommended spacing between lines: the sum of the metrics'
    /// descent, ascent, and leading.
    pub fn spacing(&self) -> f32 {
        unsafe { sys::SkFont_getSpacing(self.0) }
    }
}

impl Default for Font {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Font_delete(self.0) };
    }
}
