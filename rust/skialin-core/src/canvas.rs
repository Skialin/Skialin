use std::marker::PhantomData;

use crate::paint::BlendMode;
use crate::path::Path;
use crate::{sys, Color, Image, Matrix, Paint, Point, RRect, Rect, SamplingOptions, TextBlob, Vertices, M44};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PointMode {
    Points,
    Lines,
    Polygon,
}

impl From<PointMode> for sys::SkCanvas_PointMode {
    fn from(mode: PointMode) -> Self {
        (match mode {
            PointMode::Points => sys::SkCanvas_PointMode_kPoints_PointMode,
            PointMode::Lines => sys::SkCanvas_PointMode_kLines_PointMode,
            PointMode::Polygon => sys::SkCanvas_PointMode_kPolygon_PointMode,
        }) as sys::SkCanvas_PointMode
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SrcRectConstraint {
    Strict,
    Fast,
}

impl From<SrcRectConstraint> for sys::SkCanvas_SrcRectConstraint {
    fn from(constraint: SrcRectConstraint) -> Self {
        (match constraint {
            SrcRectConstraint::Strict => sys::SkCanvas_SrcRectConstraint_kStrict_SrcRectConstraint,
            SrcRectConstraint::Fast => sys::SkCanvas_SrcRectConstraint_kFast_SrcRectConstraint,
        }) as sys::SkCanvas_SrcRectConstraint
    }
}

fn to_sk_sampling(sampling: SamplingOptions) -> sys::SkSamplingOptions {
    let (cubic_b, cubic_c) = sampling.cubic.unwrap_or((0.0, 0.0));
    sys::SkSamplingOptions {
        maxAniso: sampling.max_aniso,
        useCubic: sampling.cubic.is_some(),
        cubic: sys::SkCubicResampler { B: cubic_b, C: cubic_c },
        filter: sampling.filter.into(),
        mipmap: sampling.mipmap.into(),
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ClipOp {
    Difference,
    Intersect,
}

impl From<ClipOp> for sys::SkClipOp {
    fn from(op: ClipOp) -> Self {
        (match op {
            ClipOp::Difference => 0,
            ClipOp::Intersect => 1,
        }) as sys::SkClipOp
    }
}

/// Borrowed for the lifetime of the [`crate::Surface`] it was obtained from.
pub struct Canvas<'a> {
    pub(crate) ptr: *mut sys::SkCanvas,
    pub(crate) _marker: PhantomData<&'a mut ()>,
}

impl<'a> Canvas<'a> {
    /// # Safety
    /// `ptr` must point to a live `SkCanvas` for the duration of `'a`.
    pub unsafe fn from_raw(ptr: *mut sys::SkCanvas) -> Self {
        Canvas { ptr, _marker: PhantomData }
    }

    pub fn as_raw(&self) -> *mut sys::SkCanvas {
        self.ptr
    }

    fn as_mut(&mut self) -> &mut sys::SkCanvas {
        unsafe { &mut *self.ptr }
    }

    pub fn clear(&mut self, color: Color) {
        unsafe { self.as_mut().clear(color) };
    }

    pub fn draw_color(&mut self, color: Color, mode: BlendMode) {
        unsafe { self.as_mut().drawColor(color, mode.into()) };
    }

    pub fn draw_paint(&mut self, paint: &Paint) {
        unsafe { self.as_mut().drawPaint(&*paint.0) };
    }

    pub fn draw_line(&mut self, p0: Point, p1: Point, paint: &Paint) {
        unsafe { self.as_mut().drawLine1(p0.into(), p1.into(), &*paint.0) };
    }

    pub fn draw_rect(&mut self, rect: Rect, paint: &Paint) {
        let sk_rect: sys::SkRect = rect.into();
        unsafe { self.as_mut().drawRect(&sk_rect, &*paint.0) };
    }

    pub fn draw_oval(&mut self, rect: Rect, paint: &Paint) {
        let sk_rect: sys::SkRect = rect.into();
        unsafe { self.as_mut().drawOval(&sk_rect, &*paint.0) };
    }

    pub fn draw_circle(&mut self, center: Point, radius: f32, paint: &Paint) {
        unsafe { self.as_mut().drawCircle1(center.into(), radius, &*paint.0) };
    }

    pub fn draw_path(&mut self, path: &Path, paint: &Paint) {
        unsafe { self.as_mut().drawPath(path.0, &*paint.0) };
    }

    pub fn draw_text_blob(&mut self, blob: &TextBlob, x: f32, y: f32, paint: &Paint) {
        unsafe { self.as_mut().drawTextBlob(blob.0, x, y, &*paint.0) };
    }

    pub fn save(&mut self) -> i32 {
        unsafe { self.as_mut().save() }
    }

    pub fn restore(&mut self) {
        unsafe { self.as_mut().restore() };
    }

    pub fn restore_to_count(&mut self, save_count: i32) {
        unsafe { self.as_mut().restoreToCount(save_count) };
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        unsafe { self.as_mut().translate(dx, dy) };
    }

    pub fn scale(&mut self, sx: f32, sy: f32) {
        unsafe { self.as_mut().scale(sx, sy) };
    }

    pub fn rotate(&mut self, degrees: f32) {
        unsafe { self.as_mut().rotate(degrees) };
    }

    pub fn concat(&mut self, matrix: &Matrix) {
        unsafe { self.as_mut().concat(&matrix.0) };
    }

    pub fn clip_rect(&mut self, rect: Rect, op: ClipOp) {
        let sk_rect: sys::SkRect = rect.into();
        unsafe { self.as_mut().clipRect1(&sk_rect, op.into()) };
    }

    pub fn clip_path(&mut self, path: &Path, op: ClipOp) {
        unsafe { self.as_mut().clipPath1(path.0, op.into()) };
    }

    pub fn skew(&mut self, sx: f32, sy: f32) {
        unsafe { self.as_mut().skew(sx, sy) };
    }

    pub fn reset_matrix(&mut self) {
        unsafe { self.as_mut().resetMatrix() };
    }

    pub fn set_matrix(&mut self, matrix: &Matrix) {
        unsafe { self.as_mut().setMatrix1(&matrix.0) };
    }

    pub fn total_matrix(&self) -> Matrix {
        let mut out = Matrix::identity();
        unsafe { sys::skialin_bridge_Canvas_getTotalMatrix(self.ptr, &mut out.0) };
        out
    }

    pub fn quick_reject_rect(&self, rect: Rect) -> bool {
        let sk_rect: sys::SkRect = rect.into();
        unsafe { (*self.ptr).quickReject(&sk_rect) }
    }

    pub fn quick_reject_path(&self, path: &Path) -> bool {
        unsafe { (*self.ptr).quickReject1(path.0) }
    }

    pub fn draw_round_rect(&mut self, rect: Rect, rx: f32, ry: f32, paint: &Paint) {
        let sk_rect: sys::SkRect = rect.into();
        unsafe { self.as_mut().drawRoundRect(&sk_rect, rx, ry, &*paint.0) };
    }

    pub fn draw_arc(&mut self, oval: Rect, start_angle: f32, sweep_angle: f32, use_center: bool, paint: &Paint) {
        let sk_rect: sys::SkRect = oval.into();
        unsafe { self.as_mut().drawArc(&sk_rect, start_angle, sweep_angle, use_center, &*paint.0) };
    }

    pub fn draw_points(&mut self, mode: PointMode, points: &[Point], paint: &Paint) {
        let sk_points: Vec<sys::SkPoint> = points.iter().map(|&p| p.into()).collect();
        let span = sys::SkSpan { _phantom_0: std::marker::PhantomData, fPtr: sk_points.as_ptr().cast_mut(), fSize: sk_points.len() };
        unsafe { self.as_mut().drawPoints(mode.into(), span, &*paint.0) };
    }

    pub fn draw_image(&mut self, image: &Image, x: f32, y: f32, sampling: SamplingOptions, paint: Option<&Paint>) {
        let sk_sampling = to_sk_sampling(sampling);
        let paint_ptr = paint.map_or(std::ptr::null(), |p| &*p.0 as *const sys::SkPaint);
        unsafe { self.as_mut().drawImage2(image.0, x, y, &sk_sampling, paint_ptr) };
    }

    /// `src` defaults to the whole image when `None`.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_image_rect(&mut self, image: &Image, src: Option<Rect>, dst: Rect, sampling: SamplingOptions, paint: Option<&Paint>, constraint: SrcRectConstraint) {
        let sk_dst: sys::SkRect = dst.into();
        let sk_sampling = to_sk_sampling(sampling);
        let paint_ptr = paint.map_or(std::ptr::null(), |p| &*p.0 as *const sys::SkPaint);
        match src {
            // `src` is a required C++ reference in this overload, so it can never be passed as null.
            Some(src) => {
                let sk_src: sys::SkRect = src.into();
                unsafe { self.as_mut().drawImageRect(image.0, &sk_src, &sk_dst, &sk_sampling, paint_ptr, constraint.into()) };
            }
            None => unsafe { self.as_mut().drawImageRect1(image.0, &sk_dst, &sk_sampling, paint_ptr) },
        }
    }

    /// Saves the canvas state, then redirects drawing to a new layer.
    /// `bounds`, if given, is a hint for the layer's extent. Returns the
    /// new save count, for [`Self::restore_to_count`].
    pub fn save_layer(&mut self, bounds: Option<Rect>, paint: Option<&Paint>) -> i32 {
        let sk_bounds: Option<sys::SkRect> = bounds.map(Into::into);
        let bounds_ptr = sk_bounds.as_ref().map_or(std::ptr::null(), |r| r as *const sys::SkRect);
        let paint_ptr = paint.map_or(std::ptr::null(), |p| &*p.0 as *const sys::SkPaint);
        unsafe { self.as_mut().saveLayer(bounds_ptr, paint_ptr) }
    }

    pub fn draw_rrect(&mut self, rrect: &RRect, paint: &Paint) {
        unsafe { sys::skialin_bridge_Canvas_drawRRect(self.ptr, rrect.0, &*paint.0) };
    }

    /// Draws the ring between `outer` and `inner`; `inner` must be contained within `outer`.
    pub fn draw_drrect(&mut self, outer: &RRect, inner: &RRect, paint: &Paint) {
        unsafe { sys::skialin_bridge_Canvas_drawDRRect(self.ptr, outer.0, inner.0, &*paint.0) };
    }

    pub fn clip_rrect(&mut self, rrect: &RRect, op: ClipOp) {
        unsafe { sys::skialin_bridge_Canvas_clipRRect(self.ptr, rrect.0, op.into()) };
    }

    pub fn draw_vertices(&mut self, vertices: &Vertices, mode: BlendMode, paint: &Paint) {
        unsafe { sys::skialin_bridge_Canvas_drawVertices(self.ptr, vertices.0, mode.into(), &*paint.0) };
    }

    /// Concatenates a 4x4 local-to-device transform onto the canvas's current matrix.
    pub fn concat_44(&mut self, matrix: &M44) {
        unsafe { sys::skialin_bridge_Canvas_concat44(self.ptr, matrix.0) };
    }
}
