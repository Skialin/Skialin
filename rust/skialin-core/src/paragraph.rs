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

#[derive(Debug, Clone, PartialEq)]
pub struct GlyphInfo {
    pub bounds: crate::Rect,
    pub grapheme_cluster_range: std::ops::Range<usize>,
    pub direction: crate::TextDirection,
    pub is_ellipsis: bool,
}

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

    pub fn unresolved_glyphs(&mut self) -> Option<i32> {
        let count = unsafe { sys::skialin_bridge_Paragraph_unresolvedGlyphs(self.0) };
        (count >= 0).then_some(count)
    }

    /// The codepoints skparagraph could not resolve to a glyph during shaping - the actual
    /// characters behind [`Self::unresolved_glyphs`]'s count, for font-fallback-registry lookups.
    pub fn unresolved_codepoints(&mut self) -> Vec<i32> {
        let count = unsafe { sys::skialin_bridge_Paragraph_unresolvedCodepoints(self.0, std::ptr::null_mut(), 0) };
        if count <= 0 {
            return Vec::new();
        }
        let mut buf = vec![0i32; count as usize];
        unsafe { sys::skialin_bridge_Paragraph_unresolvedCodepoints(self.0, buf.as_mut_ptr(), count) };
        buf
    }

    /// Invalidates cached layout state so the next [`Self::layout`] call redoes
    /// shaping/positioning instead of being a no-op for an unchanged width.
    pub fn mark_dirty(&mut self) {
        unsafe { sys::skialin_bridge_Paragraph_markDirty(self.0) };
    }

    pub fn glyph_position_at_coordinate(&mut self, dx: f32, dy: f32) -> GlyphPosition {
        let mut affinity = 0i32;
        let position = unsafe { sys::skialin_bridge_Paragraph_getGlyphPositionAtCoordinate(self.0, dx, dy, &mut affinity) };
        GlyphPosition { position, affinity: if affinity == 0 { Affinity::Upstream } else { Affinity::Downstream } }
    }

    pub fn word_boundary(&mut self, offset: u32) -> std::ops::Range<usize> {
        let (mut start, mut end) = (0usize, 0usize);
        unsafe { sys::skialin_bridge_Paragraph_getWordBoundary(self.0, offset, &mut start, &mut end) };
        start..end
    }

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

    pub fn line_metrics(&mut self) -> Vec<LineMetrics> {
        let count = self.line_number();
        (0..count as i32).filter_map(|i| self.line_metrics_at(i)).collect()
    }

    pub fn rects_for_range(&mut self, start: u32, end: u32, height_style: RectHeightStyle, width_style: RectWidthStyle) -> Vec<TextBox> {
        let count = unsafe { sys::skialin_bridge_Paragraph_getRectsForRange(self.0, start, end, height_style.into(), width_style.into(), std::ptr::null_mut(), 0) };
        if count <= 0 {
            return Vec::new();
        }
        let mut buf = vec![0f32; count as usize * 5];
        unsafe { sys::skialin_bridge_Paragraph_getRectsForRange(self.0, start, end, height_style.into(), width_style.into(), buf.as_mut_ptr(), count) };
        buf.chunks_exact(5)
            .map(|c| TextBox {
                rect: crate::Rect::new(c[0], c[1], c[2], c[3]),
                direction: if c[4] > 0.5 { crate::TextDirection::Ltr } else { crate::TextDirection::Rtl },
            })
            .collect()
    }

    pub fn rects_for_placeholders(&mut self) -> Vec<TextBox> {
        let count = unsafe { sys::skialin_bridge_Paragraph_getRectsForPlaceholders(self.0, std::ptr::null_mut(), 0) };
        if count <= 0 {
            return Vec::new();
        }
        let mut buf = vec![0f32; count as usize * 5];
        unsafe { sys::skialin_bridge_Paragraph_getRectsForPlaceholders(self.0, buf.as_mut_ptr(), count) };
        buf.chunks_exact(5)
            .map(|c| TextBox {
                rect: crate::Rect::new(c[0], c[1], c[2], c[3]),
                direction: if c[4] > 0.5 { crate::TextDirection::Ltr } else { crate::TextDirection::Rtl },
            })
            .collect()
    }

    pub fn glyph_info_at_utf16_offset(&mut self, code_unit_index: usize) -> Option<GlyphInfo> {
        let mut bounds = [0f32; 4];
        let (mut range_start, mut range_end) = (0usize, 0usize);
        let mut direction = 0i32;
        let mut is_ellipsis = false;
        let found = unsafe {
            sys::skialin_bridge_Paragraph_getGlyphInfoAtUTF16Offset(self.0, code_unit_index, bounds.as_mut_ptr(), &mut range_start, &mut range_end, &mut direction, &mut is_ellipsis)
        };
        found.then_some(GlyphInfo {
            bounds: crate::Rect::new(bounds[0], bounds[1], bounds[2], bounds[3]),
            grapheme_cluster_range: range_start..range_end,
            direction: if direction != 0 { crate::TextDirection::Ltr } else { crate::TextDirection::Rtl },
            is_ellipsis,
        })
    }

    pub fn closest_glyph_info_at(&mut self, dx: f32, dy: f32) -> Option<GlyphInfo> {
        let mut bounds = [0f32; 4];
        let (mut range_start, mut range_end) = (0usize, 0usize);
        let mut direction = 0i32;
        let mut is_ellipsis = false;
        let found = unsafe { sys::skialin_bridge_Paragraph_getClosestUTF16GlyphInfoAt(self.0, dx, dy, bounds.as_mut_ptr(), &mut range_start, &mut range_end, &mut direction, &mut is_ellipsis) };
        found.then_some(GlyphInfo {
            bounds: crate::Rect::new(bounds[0], bounds[1], bounds[2], bounds[3]),
            grapheme_cluster_range: range_start..range_end,
            direction: if direction != 0 { crate::TextDirection::Ltr } else { crate::TextDirection::Rtl },
            is_ellipsis,
        })
    }

    pub fn update_font_size(&mut self, from: usize, to: usize, font_size: f32) {
        unsafe { sys::skialin_bridge_Paragraph_updateFontSize(self.0, from, to, font_size) };
    }

    pub fn update_foreground_paint(&mut self, from: usize, to: usize, paint: &crate::Paint) {
        unsafe { sys::skialin_bridge_Paragraph_updateForegroundPaint(self.0, from, to, &*paint.0) };
    }

    pub fn update_background_paint(&mut self, from: usize, to: usize, paint: &crate::Paint) {
        unsafe { sys::skialin_bridge_Paragraph_updateBackgroundPaint(self.0, from, to, &*paint.0) };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RectHeightStyle {
    Tight,
    Max,
    IncludeLineSpacingMiddle,
    IncludeLineSpacingTop,
    IncludeLineSpacingBottom,
    Strut,
}

impl From<RectHeightStyle> for i32 {
    fn from(style: RectHeightStyle) -> Self {
        match style {
            RectHeightStyle::Tight => 0,
            RectHeightStyle::Max => 1,
            RectHeightStyle::IncludeLineSpacingMiddle => 2,
            RectHeightStyle::IncludeLineSpacingTop => 3,
            RectHeightStyle::IncludeLineSpacingBottom => 4,
            RectHeightStyle::Strut => 5,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RectWidthStyle {
    Tight,
    Max,
}

impl From<RectWidthStyle> for i32 {
    fn from(style: RectWidthStyle) -> Self {
        match style {
            RectWidthStyle::Tight => 0,
            RectWidthStyle::Max => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextBox {
    pub rect: crate::Rect,
    pub direction: crate::TextDirection,
}

impl Drop for Paragraph {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Paragraph_delete(self.0) };
    }
}
