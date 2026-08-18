use crate::{sys, Color};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PaintStyle {
    Fill,
    Stroke,
    StrokeAndFill,
}

impl From<PaintStyle> for sys::SkPaint_Style {
    fn from(style: PaintStyle) -> Self {
        match style {
            PaintStyle::Fill => sys::SkPaint_Style_kFill_Style,
            PaintStyle::Stroke => sys::SkPaint_Style_kStroke_Style,
            PaintStyle::StrokeAndFill => sys::SkPaint_Style_kStrokeAndFill_Style,
        }
    }
}

impl From<sys::SkPaint_Style> for PaintStyle {
    fn from(style: sys::SkPaint_Style) -> Self {
        match style {
            sys::SkPaint_Style_kStroke_Style => PaintStyle::Stroke,
            sys::SkPaint_Style_kStrokeAndFill_Style => PaintStyle::StrokeAndFill,
            _ => PaintStyle::Fill,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StrokeCap {
    Butt,
    Round,
    Square,
}

impl From<StrokeCap> for sys::SkPaint_Cap {
    fn from(cap: StrokeCap) -> Self {
        match cap {
            StrokeCap::Butt => sys::SkPaint_Cap_kButt_Cap,
            StrokeCap::Round => sys::SkPaint_Cap_kRound_Cap,
            StrokeCap::Square => sys::SkPaint_Cap_kSquare_Cap,
        }
    }
}

impl From<sys::SkPaint_Cap> for StrokeCap {
    fn from(cap: sys::SkPaint_Cap) -> Self {
        match cap {
            sys::SkPaint_Cap_kRound_Cap => StrokeCap::Round,
            sys::SkPaint_Cap_kSquare_Cap => StrokeCap::Square,
            _ => StrokeCap::Butt,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StrokeJoin {
    Miter,
    Round,
    Bevel,
}

impl From<StrokeJoin> for sys::SkPaint_Join {
    fn from(join: StrokeJoin) -> Self {
        match join {
            StrokeJoin::Miter => sys::SkPaint_Join_kMiter_Join,
            StrokeJoin::Round => sys::SkPaint_Join_kRound_Join,
            StrokeJoin::Bevel => sys::SkPaint_Join_kBevel_Join,
        }
    }
}

impl From<sys::SkPaint_Join> for StrokeJoin {
    fn from(join: sys::SkPaint_Join) -> Self {
        match join {
            sys::SkPaint_Join_kRound_Join => StrokeJoin::Round,
            sys::SkPaint_Join_kBevel_Join => StrokeJoin::Bevel,
            _ => StrokeJoin::Miter,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BlendMode {
    Clear,
    Src,
    Dst,
    SrcOver,
    DstOver,
    SrcIn,
    DstIn,
    SrcOut,
    DstOut,
    SrcAtop,
    DstAtop,
    Xor,
    Plus,
    Modulate,
    Screen,
}

impl From<BlendMode> for sys::SkBlendMode {
    fn from(mode: BlendMode) -> Self {
        use BlendMode::*;
        (match mode {
            Clear => sys::SkBlendMode_kClear,
            Src => sys::SkBlendMode_kSrc,
            Dst => sys::SkBlendMode_kDst,
            SrcOver => sys::SkBlendMode_kSrcOver,
            DstOver => sys::SkBlendMode_kDstOver,
            SrcIn => sys::SkBlendMode_kSrcIn,
            DstIn => sys::SkBlendMode_kDstIn,
            SrcOut => sys::SkBlendMode_kSrcOut,
            DstOut => sys::SkBlendMode_kDstOut,
            SrcAtop => sys::SkBlendMode_kSrcATop,
            DstAtop => sys::SkBlendMode_kDstATop,
            Xor => sys::SkBlendMode_kXor,
            Plus => sys::SkBlendMode_kPlus,
            Modulate => sys::SkBlendMode_kModulate,
            Screen => sys::SkBlendMode_kScreen,
        }) as sys::SkBlendMode
    }
}

pub struct Paint(pub(crate) sys::SkPaint);

impl Paint {
    pub fn new() -> Self {
        Paint(unsafe { sys::SkPaint::new() })
    }

    pub fn color(&self) -> Color {
        unsafe { self.0.getColor() }
    }

    pub fn set_color(&mut self, color: Color) -> &mut Self {
        unsafe { self.0.setColor(color) };
        self
    }

    pub fn is_anti_alias(&self) -> bool {
        unsafe { self.0.isAntiAlias() }
    }

    pub fn set_anti_alias(&mut self, anti_alias: bool) -> &mut Self {
        unsafe { self.0.setAntiAlias(anti_alias) };
        self
    }

    pub fn style(&self) -> PaintStyle {
        unsafe { self.0.getStyle() }.into()
    }

    pub fn set_style(&mut self, style: PaintStyle) -> &mut Self {
        unsafe { self.0.setStyle(style.into()) };
        self
    }

    pub fn stroke_width(&self) -> f32 {
        unsafe { self.0.getStrokeWidth() }
    }

    pub fn set_stroke_width(&mut self, width: f32) -> &mut Self {
        unsafe { self.0.setStrokeWidth(width) };
        self
    }

    pub fn stroke_cap(&self) -> StrokeCap {
        unsafe { self.0.getStrokeCap() }.into()
    }

    pub fn set_stroke_cap(&mut self, cap: StrokeCap) -> &mut Self {
        unsafe { self.0.setStrokeCap(cap.into()) };
        self
    }

    pub fn stroke_join(&self) -> StrokeJoin {
        unsafe { self.0.getStrokeJoin() }.into()
    }

    pub fn set_stroke_join(&mut self, join: StrokeJoin) -> &mut Self {
        unsafe { self.0.setStrokeJoin(join.into()) };
        self
    }

    pub fn set_blend_mode(&mut self, mode: BlendMode) -> &mut Self {
        unsafe { self.0.setBlendMode(mode.into()) };
        self
    }
}

impl Default for Paint {
    fn default() -> Self {
        Paint::new()
    }
}

impl Drop for Paint {
    fn drop(&mut self) {
        unsafe { self.0.destruct() };
    }
}
