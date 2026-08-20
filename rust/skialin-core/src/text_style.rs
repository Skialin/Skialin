use crate::{sys, Color, Edging, FontStyle, Hinting, Typeface};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextDecoration(pub i32);

impl TextDecoration {
    pub const NONE: Self = TextDecoration(0x0);
    pub const UNDERLINE: Self = TextDecoration(0x1);
    pub const OVERLINE: Self = TextDecoration(0x2);
    pub const LINE_THROUGH: Self = TextDecoration(0x4);
}

impl std::ops::BitOr for TextDecoration {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        TextDecoration(self.0 | rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDecorationMode {
    Gaps,
    Through,
}

impl From<TextDecorationMode> for i32 {
    fn from(mode: TextDecorationMode) -> Self {
        match mode {
            TextDecorationMode::Gaps => 0,
            TextDecorationMode::Through => 1,
        }
    }
}

impl From<i32> for TextDecorationMode {
    fn from(value: i32) -> Self {
        match value {
            0 => TextDecorationMode::Gaps,
            _ => TextDecorationMode::Through,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDecorationStyle {
    Solid,
    Double,
    Dotted,
    Dashed,
    Wavy,
}

impl From<TextDecorationStyle> for i32 {
    fn from(style: TextDecorationStyle) -> Self {
        match style {
            TextDecorationStyle::Solid => 0,
            TextDecorationStyle::Double => 1,
            TextDecorationStyle::Dotted => 2,
            TextDecorationStyle::Dashed => 3,
            TextDecorationStyle::Wavy => 4,
        }
    }
}

impl From<i32> for TextDecorationStyle {
    fn from(value: i32) -> Self {
        match value {
            1 => TextDecorationStyle::Double,
            2 => TextDecorationStyle::Dotted,
            3 => TextDecorationStyle::Dashed,
            4 => TextDecorationStyle::Wavy,
            _ => TextDecorationStyle::Solid,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shadow {
    pub color: Color,
    pub offset_x: f32,
    pub offset_y: f32,
    pub blur_sigma: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontFeature {
    pub name: String,
    pub value: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Decoration {
    pub decoration: TextDecoration,
    pub mode: TextDecorationMode,
    pub color: Color,
    pub style: TextDecorationStyle,
    pub thickness_multiplier: f32,
}

pub struct TextStyle(pub(crate) *mut sys::skia::textlayout::TextStyle);

impl TextStyle {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::skia::textlayout::TextStyle) -> Self {
        TextStyle(ptr)
    }

    pub fn new() -> Self {
        unsafe { Self::from_raw(sys::skialin_bridge_TextStyle_new()) }
    }

    pub fn clone_style(&self) -> Self {
        unsafe { Self::from_raw(sys::skialin_bridge_TextStyle_clone(self.0)) }
    }

    pub fn color(&self) -> Color {
        unsafe { sys::skialin_bridge_TextStyle_getColor(self.0) }
    }

    pub fn set_color(&mut self, color: Color) {
        unsafe { sys::skialin_bridge_TextStyle_setColor(self.0, color) };
    }

    pub fn font_families(&self) -> Vec<String> {
        let count = unsafe { sys::skialin_bridge_TextStyle_countFontFamilies(self.0) };
        (0..count)
            .map(|i| {
                let data = unsafe { crate::Data::from_raw(sys::skialin_bridge_TextStyle_fontFamily(self.0, i)) }.expect("fontFamily never returns null");
                String::from_utf8_lossy(data.as_bytes()).into_owned()
            })
            .collect()
    }

    pub fn set_font_families(&mut self, families: &[&str]) {
        let ptrs: Vec<*const std::os::raw::c_char> = families.iter().map(|s| s.as_ptr().cast()).collect();
        let lens: Vec<usize> = families.iter().map(|s| s.len()).collect();
        unsafe { sys::skialin_bridge_TextStyle_setFontFamilies(self.0, ptrs.as_ptr(), lens.as_ptr(), families.len()) };
    }

    pub fn font_size(&self) -> f32 {
        unsafe { sys::skialin_bridge_TextStyle_getFontSize(self.0) }
    }

    pub fn set_font_size(&mut self, size: f32) {
        unsafe { sys::skialin_bridge_TextStyle_setFontSize(self.0, size) };
    }

    pub fn font_style(&self) -> FontStyle {
        let (mut weight, mut width, mut slant) = (0, 0, 0);
        unsafe { sys::skialin_bridge_TextStyle_getFontStyle(self.0, &mut weight, &mut width, &mut slant) };
        FontStyle::new(weight, width, slant.into())
    }

    pub fn set_font_style(&mut self, style: FontStyle) {
        unsafe { sys::skialin_bridge_TextStyle_setFontStyle(self.0, style.weight, style.width, style.slant.into()) };
    }

    pub fn decoration(&self) -> Decoration {
        let (mut ty, mut mode, mut color, mut style, mut thickness) = (0, 0, 0, 0, 0.0);
        unsafe { sys::skialin_bridge_TextStyle_getDecoration(self.0, &mut ty, &mut mode, &mut color, &mut style, &mut thickness) };
        Decoration { decoration: TextDecoration(ty), mode: mode.into(), color, style: style.into(), thickness_multiplier: thickness }
    }

    pub fn set_decoration(&mut self, decoration: TextDecoration) {
        unsafe { sys::skialin_bridge_TextStyle_setDecoration(self.0, decoration.0) };
    }

    pub fn set_decoration_mode(&mut self, mode: TextDecorationMode) {
        unsafe { sys::skialin_bridge_TextStyle_setDecorationMode(self.0, mode.into()) };
    }

    pub fn set_decoration_color(&mut self, color: Color) {
        unsafe { sys::skialin_bridge_TextStyle_setDecorationColor(self.0, color) };
    }

    pub fn set_decoration_style(&mut self, style: TextDecorationStyle) {
        unsafe { sys::skialin_bridge_TextStyle_setDecorationStyle(self.0, style.into()) };
    }

    pub fn set_decoration_thickness_multiplier(&mut self, multiplier: f32) {
        unsafe { sys::skialin_bridge_TextStyle_setDecorationThicknessMultiplier(self.0, multiplier) };
    }

    pub fn letter_spacing(&self) -> f32 {
        unsafe { sys::skialin_bridge_TextStyle_getLetterSpacing(self.0) }
    }

    pub fn set_letter_spacing(&mut self, letter_spacing: f32) {
        unsafe { sys::skialin_bridge_TextStyle_setLetterSpacing(self.0, letter_spacing) };
    }

    pub fn word_spacing(&self) -> f32 {
        unsafe { sys::skialin_bridge_TextStyle_getWordSpacing(self.0) }
    }

    pub fn set_word_spacing(&mut self, word_spacing: f32) {
        unsafe { sys::skialin_bridge_TextStyle_setWordSpacing(self.0, word_spacing) };
    }

    pub fn height(&self) -> f32 {
        unsafe { sys::skialin_bridge_TextStyle_getHeight(self.0) }
    }

    pub fn set_height(&mut self, height: f32) {
        unsafe { sys::skialin_bridge_TextStyle_setHeight(self.0, height) };
    }

    pub fn height_override(&self) -> bool {
        unsafe { sys::skialin_bridge_TextStyle_getHeightOverride(self.0) }
    }

    pub fn set_height_override(&mut self, height_override: bool) {
        unsafe { sys::skialin_bridge_TextStyle_setHeightOverride(self.0, height_override) };
    }

    /// Baseline offset in px, applied on top of the run's normal baseline (e.g. for
    /// superscript/subscript positioning).
    pub fn baseline_shift(&self) -> f32 {
        unsafe { sys::skialin_bridge_TextStyle_getBaselineShift(self.0) }
    }

    pub fn set_baseline_shift(&mut self, baseline_shift: f32) {
        unsafe { sys::skialin_bridge_TextStyle_setBaselineShift(self.0, baseline_shift) };
    }

    /// Whether extra line-height leading is split evenly above/below the run (true) or added
    /// entirely below per legacy behavior (false).
    pub fn half_leading(&self) -> bool {
        unsafe { sys::skialin_bridge_TextStyle_getHalfLeading(self.0) }
    }

    pub fn set_half_leading(&mut self, half_leading: bool) {
        unsafe { sys::skialin_bridge_TextStyle_setHalfLeading(self.0, half_leading) };
    }

    /// Per-run rasterization edging, forwarded to the `SkFont` skparagraph builds internally
    /// for this run.
    pub fn font_edging(&self) -> Edging {
        Edging::from_raw(unsafe { sys::skialin_bridge_TextStyle_getFontEdging(self.0) })
    }

    pub fn set_font_edging(&mut self, edging: Edging) {
        unsafe { sys::skialin_bridge_TextStyle_setFontEdging(self.0, edging.as_raw()) };
    }

    /// Per-run rasterization hinting, forwarded to the `SkFont` skparagraph builds internally
    /// for this run.
    pub fn font_hinting(&self) -> Hinting {
        Hinting::from_raw_i32(unsafe { sys::skialin_bridge_TextStyle_getFontHinting(self.0) })
    }

    pub fn set_font_hinting(&mut self, hinting: Hinting) {
        unsafe { sys::skialin_bridge_TextStyle_setFontHinting(self.0, hinting.as_raw_i32()) };
    }

    pub fn shadows(&self) -> Vec<Shadow> {
        let count = unsafe { sys::skialin_bridge_TextStyle_getShadows(self.0, std::ptr::null_mut(), std::ptr::null_mut(), 0) };
        if count <= 0 {
            return Vec::new();
        }
        let mut colors = vec![0u32; count as usize];
        let mut floats = vec![0f32; count as usize * 3];
        unsafe { sys::skialin_bridge_TextStyle_getShadows(self.0, colors.as_mut_ptr(), floats.as_mut_ptr(), count) };
        colors
            .into_iter()
            .zip(floats.chunks_exact(3))
            .map(|(color, c)| Shadow { color, offset_x: c[0], offset_y: c[1], blur_sigma: c[2] as f64 })
            .collect()
    }

    pub fn add_shadow(&mut self, shadow: Shadow) {
        unsafe { sys::skialin_bridge_TextStyle_addShadow(self.0, shadow.color, shadow.offset_x, shadow.offset_y, shadow.blur_sigma) };
    }

    pub fn reset_shadows(&mut self) {
        unsafe { sys::skialin_bridge_TextStyle_resetShadows(self.0) };
    }

    pub fn font_features(&self) -> Vec<FontFeature> {
        let count = unsafe { sys::skialin_bridge_TextStyle_countFontFeatures(self.0) };
        (0..count)
            .map(|i| {
                let name = unsafe { crate::Data::from_raw(sys::skialin_bridge_TextStyle_fontFeatureName(self.0, i)) }.expect("fontFeatureName never returns null");
                let value = unsafe { sys::skialin_bridge_TextStyle_fontFeatureValue(self.0, i) };
                FontFeature { name: String::from_utf8_lossy(name.as_bytes()).into_owned(), value }
            })
            .collect()
    }

    pub fn add_font_feature(&mut self, name: &str, value: i32) {
        unsafe { sys::skialin_bridge_TextStyle_addFontFeature(self.0, name.as_ptr().cast(), name.len(), value) };
    }

    pub fn reset_font_features(&mut self) {
        unsafe { sys::skialin_bridge_TextStyle_resetFontFeatures(self.0) };
    }

    pub fn has_foreground(&self) -> bool {
        unsafe { sys::skialin_bridge_TextStyle_hasForeground(self.0) }
    }

    pub fn has_background(&self) -> bool {
        unsafe { sys::skialin_bridge_TextStyle_hasBackground(self.0) }
    }

    pub fn foreground(&self) -> Option<crate::Paint> {
        if !self.has_foreground() {
            return None;
        }
        let mut paint = crate::Paint::new();
        unsafe { sys::skialin_bridge_TextStyle_getForegroundPaint(self.0, &mut *paint.0) };
        Some(paint)
    }

    pub fn background(&self) -> Option<crate::Paint> {
        if !self.has_background() {
            return None;
        }
        let mut paint = crate::Paint::new();
        unsafe { sys::skialin_bridge_TextStyle_getBackgroundPaint(self.0, &mut *paint.0) };
        Some(paint)
    }

    pub fn set_foreground_paint(&mut self, paint: &crate::Paint) {
        unsafe { sys::skialin_bridge_TextStyle_setForegroundPaint(self.0, &*paint.0) };
    }

    pub fn set_background_paint(&mut self, paint: &crate::Paint) {
        unsafe { sys::skialin_bridge_TextStyle_setBackgroundPaint(self.0, &*paint.0) };
    }

    pub fn clear_foreground(&mut self) {
        unsafe { sys::skialin_bridge_TextStyle_clearForeground(self.0) };
    }

    pub fn clear_background(&mut self) {
        unsafe { sys::skialin_bridge_TextStyle_clearBackground(self.0) };
    }

    pub fn typeface(&self) -> Option<Typeface> {
        unsafe { Typeface::from_raw(sys::skialin_bridge_TextStyle_refTypeface(self.0)) }
    }

    pub fn set_typeface(&mut self, typeface: Option<&Typeface>) {
        let ptr = typeface.map_or(std::ptr::null_mut(), |t| t.0);
        unsafe { sys::skialin_bridge_TextStyle_setTypeface(self.0, ptr) };
    }

    pub fn locale(&self) -> String {
        let data = unsafe { crate::Data::from_raw(sys::skialin_bridge_TextStyle_getLocale(self.0)) }.expect("getLocale never returns null");
        String::from_utf8_lossy(data.as_bytes()).into_owned()
    }

    pub fn set_locale(&mut self, locale: &str) {
        unsafe { sys::skialin_bridge_TextStyle_setLocale(self.0, locale.as_ptr().cast(), locale.len()) };
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for TextStyle {
    fn clone(&self) -> Self {
        self.clone_style()
    }
}

impl Drop for TextStyle {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_TextStyle_delete(self.0) };
    }
}
