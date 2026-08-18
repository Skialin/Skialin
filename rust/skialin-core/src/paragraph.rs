use crate::{sys, Canvas};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Affinity {
    Upstream,
    Downstream,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlyphPosition {
    pub position: i32,
    pub affinity: Affinity,
}

/// Line layout metrics. Mirrors skparagraph's `LineMetrics`, excluding its
/// per-run `fLineMetrics` map.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineMetrics {
    pub start_index: usize,
    pub end_index: usize,
    pub end_excluding_whitespaces: usize,
    pub end_including_newline: usize,
    pub hard_break: bool,
    pub ascent: f64,
    pub descent: f64,
    pub unscaled_ascent: f64,
    pub height: f64,
    pub width: f64,
    pub left: f64,
    pub baseline: f64,
    pub line_number: usize,
}

pub struct Paragraph(pub(crate) *mut sys::skia::textlayout::Paragraph);

impl Paragraph {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::skia::textlayout::Paragraph) -> Self {
        Paragraph(ptr)
    }

    pub fn layout(&mut self, width: f32) {
        unsafe { sys::skialin_bridge_Paragraph_layout(self.0, width) };
    }

    pub fn paint(&mut self, canvas: &mut Canvas, x: f32, y: f32) {
        unsafe { sys::skialin_bridge_Paragraph_paint(self.0, canvas.as_raw(), x, y) };
    }

    pub fn max_width(&self) -> f32 {
        unsafe { sys::skialin_bridge_Paragraph_getMaxWidth(self.0) }
    }

    pub fn height(&self) -> f32 {
        unsafe { sys::skialin_bridge_Paragraph_getHeight(self.0) }
    }

    pub fn min_intrinsic_width(&self) -> f32 {
        unsafe { sys::skialin_bridge_Paragraph_getMinIntrinsicWidth(self.0) }
    }

    pub fn max_intrinsic_width(&self) -> f32 {
        unsafe { sys::skialin_bridge_Paragraph_getMaxIntrinsicWidth(self.0) }
    }

    pub fn alphabetic_baseline(&self) -> f32 {
        unsafe { sys::skialin_bridge_Paragraph_getAlphabeticBaseline(self.0) }
    }

    pub fn ideographic_baseline(&self) -> f32 {
        unsafe { sys::skialin_bridge_Paragraph_getIdeographicBaseline(self.0) }
    }

    pub fn longest_line(&self) -> f32 {
        unsafe { sys::skialin_bridge_Paragraph_getLongestLine(self.0) }
    }

    pub fn did_exceed_max_lines(&self) -> bool {
        unsafe { sys::skialin_bridge_Paragraph_didExceedMaxLines(self.0) }
    }

    pub fn line_number(&mut self) -> usize {
        unsafe { sys::skialin_bridge_Paragraph_lineNumber(self.0) }
    }

    /// The number of unresolved glyphs, or `None` if the paragraph hasn't
    /// been shaped yet (i.e. before the first [`layout`](Self::layout)).
    pub fn unresolved_glyphs(&mut self) -> Option<i32> {
        let count = unsafe { sys::skialin_bridge_Paragraph_unresolvedGlyphs(self.0) };
        (count >= 0).then_some(count)
    }

    /// The glyph at the given coordinate, with the paragraph's top-left as
    /// the origin and +y as down.
    pub fn glyph_position_at_coordinate(&mut self, dx: f32, dy: f32) -> GlyphPosition {
        let mut affinity = 0i32;
        let position = unsafe { sys::skialin_bridge_Paragraph_getGlyphPositionAtCoordinate(self.0, dx, dy, &mut affinity) };
        GlyphPosition { position, affinity: if affinity == 0 { Affinity::Upstream } else { Affinity::Downstream } }
    }

    /// The `[start, end)` range of the word containing the glyph at `offset`.
    pub fn word_boundary(&mut self, offset: u32) -> std::ops::Range<usize> {
        let (mut start, mut end) = (0usize, 0usize);
        unsafe { sys::skialin_bridge_Paragraph_getWordBoundary(self.0, offset, &mut start, &mut end) };
        start..end
    }

    /// Layout metrics for line `line_number` (0-indexed), or `None` if out of range.
    pub fn line_metrics_at(&self, line_number: i32) -> Option<LineMetrics> {
        let (mut start_index, mut end_index, mut end_excluding_whitespaces, mut end_including_newline) = (0usize, 0usize, 0usize, 0usize);
        let mut hard_break = 0i32;
        let (mut ascent, mut descent, mut unscaled_ascent, mut height, mut width, mut left, mut baseline) = (0f64, 0f64, 0f64, 0f64, 0f64, 0f64, 0f64);
        let found = unsafe {
            sys::skialin_bridge_Paragraph_getLineMetricsAt(
                self.0,
                line_number,
                &mut start_index,
                &mut end_index,
                &mut end_excluding_whitespaces,
                &mut end_including_newline,
                &mut hard_break,
                &mut ascent,
                &mut descent,
                &mut unscaled_ascent,
                &mut height,
                &mut width,
                &mut left,
                &mut baseline,
            )
        };
        found.then_some(LineMetrics {
            start_index,
            end_index,
            end_excluding_whitespaces,
            end_including_newline,
            hard_break: hard_break != 0,
            ascent,
            descent,
            unscaled_ascent,
            height,
            width,
            left,
            baseline,
            line_number: line_number as usize,
        })
    }

    /// Layout metrics for every line, in order.
    pub fn line_metrics(&mut self) -> Vec<LineMetrics> {
        let count = self.line_number();
        (0..count as i32).filter_map(|i| self.line_metrics_at(i)).collect()
    }
}

impl Drop for Paragraph {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Paragraph_delete(self.0) };
    }
}
