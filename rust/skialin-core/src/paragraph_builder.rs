use crate::{sys, FontCollection, Paragraph, ParagraphStyle, TextStyle};

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
