use crate::{sys, TextStyle};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDirection {
    Rtl,
    Ltr,
}

impl From<TextDirection> for i32 {
    fn from(direction: TextDirection) -> Self {
        match direction {
            TextDirection::Rtl => 0,
            TextDirection::Ltr => 1,
        }
    }
}

impl From<i32> for TextDirection {
    fn from(value: i32) -> Self {
        match value {
            1 => TextDirection::Ltr,
            _ => TextDirection::Rtl,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Right,
    Center,
    Justify,
    Start,
    End,
}

impl From<TextAlign> for i32 {
    fn from(align: TextAlign) -> Self {
        match align {
            TextAlign::Left => 0,
            TextAlign::Right => 1,
            TextAlign::Center => 2,
            TextAlign::Justify => 3,
            TextAlign::Start => 4,
            TextAlign::End => 5,
        }
    }
}

impl From<i32> for TextAlign {
    fn from(value: i32) -> Self {
        match value {
            1 => TextAlign::Right,
            2 => TextAlign::Center,
            3 => TextAlign::Justify,
            4 => TextAlign::Start,
            5 => TextAlign::End,
            _ => TextAlign::Left,
        }
    }
}

/// A bitmask controlling ascent/descent adjustments at paragraph edges.
/// Mirrors skparagraph's `TextHeightBehavior`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextHeightBehavior(pub i32);

impl TextHeightBehavior {
    pub const ALL: Self = TextHeightBehavior(0x0);
    pub const DISABLE_FIRST_ASCENT: Self = TextHeightBehavior(0x1);
    pub const DISABLE_LAST_DESCENT: Self = TextHeightBehavior(0x2);
    pub const DISABLE_ALL: Self = TextHeightBehavior(0x3);
}

pub struct ParagraphStyle(pub(crate) *mut sys::skia::textlayout::ParagraphStyle);

impl ParagraphStyle {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::skia::textlayout::ParagraphStyle) -> Self {
        ParagraphStyle(ptr)
    }

    pub fn new() -> Self {
        unsafe { Self::from_raw(sys::skialin_bridge_ParagraphStyle_new()) }
    }

    /// The key knob for RTL/bidi layout: skparagraph resolves
    /// character-level bidi via ICU internally once this is set to `Rtl`.
    pub fn text_direction(&self) -> TextDirection {
        unsafe { sys::skialin_bridge_ParagraphStyle_getTextDirection(self.0) }.into()
    }

    pub fn set_text_direction(&mut self, direction: TextDirection) {
        unsafe { sys::skialin_bridge_ParagraphStyle_setTextDirection(self.0, direction.into()) };
    }

    pub fn text_align(&self) -> TextAlign {
        unsafe { sys::skialin_bridge_ParagraphStyle_getTextAlign(self.0) }.into()
    }

    pub fn set_text_align(&mut self, align: TextAlign) {
        unsafe { sys::skialin_bridge_ParagraphStyle_setTextAlign(self.0, align.into()) };
    }

    pub fn max_lines(&self) -> usize {
        unsafe { sys::skialin_bridge_ParagraphStyle_getMaxLines(self.0) }
    }

    pub fn set_max_lines(&mut self, max_lines: usize) {
        unsafe { sys::skialin_bridge_ParagraphStyle_setMaxLines(self.0, max_lines) };
    }

    pub fn ellipsis(&self) -> String {
        let data = unsafe { crate::Data::from_raw(sys::skialin_bridge_ParagraphStyle_getEllipsis(self.0)) }.expect("getEllipsis never returns null");
        String::from_utf8_lossy(data.as_bytes()).into_owned()
    }

    pub fn set_ellipsis(&mut self, ellipsis: &str) {
        unsafe { sys::skialin_bridge_ParagraphStyle_setEllipsis(self.0, ellipsis.as_ptr().cast(), ellipsis.len()) };
    }

    pub fn height(&self) -> f32 {
        unsafe { sys::skialin_bridge_ParagraphStyle_getHeight(self.0) }
    }

    pub fn set_height(&mut self, height: f32) {
        unsafe { sys::skialin_bridge_ParagraphStyle_setHeight(self.0, height) };
    }

    pub fn text_height_behavior(&self) -> TextHeightBehavior {
        TextHeightBehavior(unsafe { sys::skialin_bridge_ParagraphStyle_getTextHeightBehavior(self.0) })
    }

    pub fn set_text_height_behavior(&mut self, behavior: TextHeightBehavior) {
        unsafe { sys::skialin_bridge_ParagraphStyle_setTextHeightBehavior(self.0, behavior.0) };
    }

    /// The default style new text runs start from.
    pub fn text_style(&self) -> TextStyle {
        unsafe { TextStyle::from_raw(sys::skialin_bridge_ParagraphStyle_getTextStyle(self.0)) }
    }

    /// `style` is copied, not consumed: it stays independently valid and closeable afterward.
    pub fn set_text_style(&mut self, style: &TextStyle) {
        unsafe { sys::skialin_bridge_ParagraphStyle_setTextStyle(self.0, style.0) };
    }
}

impl Default for ParagraphStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ParagraphStyle {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_ParagraphStyle_delete(self.0) };
    }
}
