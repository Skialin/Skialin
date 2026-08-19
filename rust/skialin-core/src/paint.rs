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

impl From<sys::SkBlendMode> for BlendMode {
    fn from(mode: sys::SkBlendMode) -> Self {
        match mode {
            sys::SkBlendMode_kClear => BlendMode::Clear,
            sys::SkBlendMode_kSrc => BlendMode::Src,
            sys::SkBlendMode_kDst => BlendMode::Dst,
            sys::SkBlendMode_kDstOver => BlendMode::DstOver,
            sys::SkBlendMode_kSrcIn => BlendMode::SrcIn,
            sys::SkBlendMode_kDstIn => BlendMode::DstIn,
            sys::SkBlendMode_kSrcOut => BlendMode::SrcOut,
            sys::SkBlendMode_kDstOut => BlendMode::DstOut,
            sys::SkBlendMode_kSrcATop => BlendMode::SrcAtop,
            sys::SkBlendMode_kDstATop => BlendMode::DstAtop,
            sys::SkBlendMode_kXor => BlendMode::Xor,
            sys::SkBlendMode_kPlus => BlendMode::Plus,
            sys::SkBlendMode_kModulate => BlendMode::Modulate,
            sys::SkBlendMode_kScreen => BlendMode::Screen,
            _ => BlendMode::SrcOver,
        }
    }
}

pub struct Paint(pub(crate) Box<sys::SkPaint>);

impl Paint {
    pub fn new() -> Self {
        Paint(crate::support::new_boxed(sys::SkPaint_SkPaint))
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

    pub fn set_shader(&mut self, shader: Option<&crate::Shader>) -> &mut Self {
        let ptr = shader.map_or(std::ptr::null_mut(), |s| s.0);
        unsafe { sys::skialin_bridge_Paint_setShader(&mut *self.0, ptr) };
        self
    }

    pub fn set_color_filter(&mut self, filter: Option<&crate::ColorFilter>) -> &mut Self {
        let ptr = filter.map_or(std::ptr::null_mut(), |f| f.0);
        unsafe { sys::skialin_bridge_Paint_setColorFilter(&mut *self.0, ptr) };
        self
    }

    pub fn set_image_filter(&mut self, filter: Option<&crate::ImageFilter>) -> &mut Self {
        let ptr = filter.map_or(std::ptr::null_mut(), |f| f.0);
        unsafe { sys::skialin_bridge_Paint_setImageFilter(&mut *self.0, ptr) };
        self
    }

    pub fn set_mask_filter(&mut self, filter: Option<&crate::MaskFilter>) -> &mut Self {
        let ptr = filter.map_or(std::ptr::null_mut(), |f| f.0);
        unsafe { sys::skialin_bridge_Paint_setMaskFilter(&mut *self.0, ptr) };
        self
    }

    pub fn set_blender(&mut self, blender: Option<&crate::Blender>) -> &mut Self {
        let ptr = blender.map_or(std::ptr::null_mut(), |b| b.0);
        unsafe { sys::skialin_bridge_Paint_setBlender(&mut *self.0, ptr) };
        self
    }

    pub fn shader(&self) -> Option<crate::Shader> {
        unsafe { crate::Shader::from_raw(sys::skialin_bridge_Paint_refShader(&*self.0)) }
    }

    pub fn color_filter(&self) -> Option<crate::ColorFilter> {
        unsafe { crate::ColorFilter::from_raw(sys::skialin_bridge_Paint_refColorFilter(&*self.0)) }
    }

    pub fn image_filter(&self) -> Option<crate::ImageFilter> {
        unsafe { crate::ImageFilter::from_raw(sys::skialin_bridge_Paint_refImageFilter(&*self.0)) }
    }

    pub fn mask_filter(&self) -> Option<crate::MaskFilter> {
        unsafe { crate::MaskFilter::from_raw(sys::skialin_bridge_Paint_refMaskFilter(&*self.0)) }
    }

    pub fn blender(&self) -> Option<crate::Blender> {
        unsafe { crate::Blender::from_raw(sys::skialin_bridge_Paint_refBlender(&*self.0)) }
    }

    pub fn set_path_effect(&mut self, effect: Option<&crate::PathEffect>) -> &mut Self {
        let ptr = effect.map_or(std::ptr::null_mut(), |e| e.0);
        unsafe { sys::skialin_bridge_Paint_setPathEffect(&mut *self.0, ptr) };
        self
    }

    pub fn path_effect(&self) -> Option<crate::PathEffect> {
        unsafe { crate::PathEffect::from_raw(sys::skialin_bridge_Paint_refPathEffect(&*self.0)) }
    }

    pub fn blend_mode(&self) -> BlendMode {
        unsafe { self.0.getBlendMode_or(sys::SkBlendMode_kSrcOver as sys::SkBlendMode) }.into()
    }

    /// Resets this paint to its default (freshly-constructed) state.
    pub fn reset(&mut self) -> &mut Self {
        unsafe { self.0.reset() };
        self
    }

    pub fn is_dither(&self) -> bool {
        unsafe { self.0.isDither() }
    }

    pub fn set_dither(&mut self, dither: bool) -> &mut Self {
        unsafe { self.0.setDither(dither) };
        self
    }

    pub fn alpha(&self) -> u8 {
        unsafe { self.0.getAlpha() }
    }

    pub fn set_alpha(&mut self, alpha: u8) -> &mut Self {
        unsafe { self.0.setAlpha(alpha.into()) };
        self
    }

    pub fn alphaf(&self) -> f32 {
        unsafe { self.0.getAlphaf() }
    }

    pub fn set_alphaf(&mut self, alpha: f32) -> &mut Self {
        unsafe { self.0.setAlphaf(alpha) };
        self
    }

    pub fn set_argb(&mut self, a: u8, r: u8, g: u8, b: u8) -> &mut Self {
        unsafe { self.0.setARGB(a.into(), r.into(), g.into(), b.into()) };
        self
    }

    pub fn stroke_miter(&self) -> f32 {
        unsafe { self.0.getStrokeMiter() }
    }

    pub fn set_stroke_miter(&mut self, miter_limit: f32) -> &mut Self {
        unsafe { self.0.setStrokeMiter(miter_limit) };
        self
    }

    /// `true` if this paint is guaranteed to draw nothing (e.g. a fully
    /// transparent color with default blend mode).
    pub fn nothing_to_draw(&self) -> bool {
        unsafe { self.0.nothingToDraw() }
    }

    /// `true` if this paint's blend mode is the default `SrcOver`.
    pub fn is_src_over(&self) -> bool {
        unsafe { self.0.isSrcOver() }
    }
}

impl Default for Paint {
    fn default() -> Self {
        Paint::new()
    }
}

impl Clone for Paint {
    fn clone(&self) -> Self {
        Paint(crate::support::new_boxed_copy(sys::SkPaint_SkPaint2, &*self.0))
    }
}

impl Drop for Paint {
    fn drop(&mut self) {
        unsafe { self.0.destruct() };
    }
}
