use crate::{sys, Color, FontStyle, Typeface};

/// A bitmask of decoration lines. Mirrors skparagraph's `TextDecoration`.
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

    /// 0 unless [`set_height_override`](Self::set_height_override) is set.
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
