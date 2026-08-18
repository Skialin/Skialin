use crate::{sys, FontCollection, Paragraph, ParagraphStyle, TextStyle};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceholderAlignment {
    /// Match the baseline of the placeholder with the text baseline.
    Baseline,
    /// The placeholder sits on top of the baseline.
    AboveBaseline,
    /// The placeholder hangs below the baseline.
    BelowBaseline,
    /// Aligned with the top edge of the font.
    Top,
    /// Aligned with the bottom edge of the font.
    Bottom,
    /// Centered within the line.
    Middle,
}

impl From<PlaceholderAlignment> for i32 {
    fn from(alignment: PlaceholderAlignment) -> Self {
        match alignment {
            PlaceholderAlignment::Baseline => 0,
            PlaceholderAlignment::AboveBaseline => 1,
            PlaceholderAlignment::BelowBaseline => 2,
            PlaceholderAlignment::Top => 3,
            PlaceholderAlignment::Bottom => 4,
            PlaceholderAlignment::Middle => 5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceholderBaseline {
    Alphabetic,
    Ideographic,
}

impl From<PlaceholderBaseline> for i32 {
    fn from(baseline: PlaceholderBaseline) -> Self {
        match baseline {
            PlaceholderBaseline::Alphabetic => 0,
            PlaceholderBaseline::Ideographic => 1,
        }
    }
}

/// Reserves space in a paragraph for the caller to draw a custom inline
/// object into. Mirrors skparagraph's `PlaceholderStyle`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlaceholderStyle {
    pub width: f32,
    pub height: f32,
    pub alignment: PlaceholderAlignment,
    pub baseline: PlaceholderBaseline,
    pub baseline_offset: f32,
}

pub struct ParagraphBuilder(*mut sys::skia::textlayout::ParagraphBuilder);

impl ParagraphBuilder {
    pub fn new(style: &ParagraphStyle, font_collection: &mut FontCollection) -> Self {
        ParagraphBuilder(unsafe { sys::skialin_bridge_ParagraphBuilder_make(style.0, font_collection.0) })
    }

    /// `style` is copied, not consumed: it stays independently valid and
    /// closeable afterward. Text added after this call, until the matching
    /// [`pop`](Self::pop), uses `style`.
    pub fn push_style(&mut self, style: &TextStyle) -> &mut Self {
        unsafe { sys::skialin_bridge_ParagraphBuilder_pushStyle(self.0, style.0) };
        self
    }

    pub fn pop(&mut self) -> &mut Self {
        unsafe { sys::skialin_bridge_ParagraphBuilder_pop(self.0) };
        self
    }

    /// Adds UTF-8 text, styled with whatever is on top of the style stack.
    pub fn add_text(&mut self, text: &str) -> &mut Self {
        unsafe { sys::skialin_bridge_ParagraphBuilder_addText(self.0, text.as_ptr().cast(), text.len()) };
        self
    }

    /// Reserves space for a custom inline object, drawn by the caller
    /// using the position/size reported by [`Paragraph::line_metrics`](crate::Paragraph::line_metrics)
    /// or the placeholder rects API. Internally adds a single object
    /// replacement character (U+FFFC).
    pub fn add_placeholder(&mut self, style: PlaceholderStyle) -> &mut Self {
        unsafe {
            sys::skialin_bridge_ParagraphBuilder_addPlaceholder(
                self.0,
                style.width,
                style.height,
                style.alignment.into(),
                style.baseline.into(),
                style.baseline_offset,
            )
        };
        self
    }

    /// Builds a [`Paragraph`] from the accumulated text and styles. The
    /// builder remains usable afterward, matching the real API.
    pub fn build(&mut self) -> Paragraph {
        unsafe { Paragraph::from_raw(sys::skialin_bridge_ParagraphBuilder_build(self.0)) }
    }
}

impl Drop for ParagraphBuilder {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_ParagraphBuilder_delete(self.0) };
    }
}
