use std::marker::PhantomData;

use crate::paint::BlendMode;
use crate::path::Path;
use crate::{sys, Bitmap, Color, Data, FilterMode, Font, Image, IRect, Matrix, Paint, Picture, Point, RRect, Rect, Region, SamplingOptions, TextBlob, Vertices, M44};

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

pub(crate) fn to_sk_sampling(sampling: SamplingOptions) -> sys::SkSamplingOptions {
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

pub struct Canvas<'a> {
    pub(crate) ptr: *mut sys::SkCanvas,
    pub(crate) _marker: PhantomData<&'a mut ()>,
}

impl<'a> Canvas<'a> {
    pub unsafe fn from_raw(ptr: *mut sys::SkCanvas) -> Self {
        Canvas { ptr, _marker: PhantomData }
    }

    pub fn new_from_bitmap(bitmap: &mut Bitmap) -> Self {
        let ptr = unsafe { sys::skialin_bridge_Canvas_newFromBitmap(bitmap.as_raw_mut()) };
        Canvas { ptr, _marker: PhantomData }
    }

    pub fn delete_owned(self) {
        unsafe { sys::skialin_bridge_Canvas_deleteOwned(self.ptr) };
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

    pub fn draw_string(&mut self, text: &str, x: f32, y: f32, font: &Font, paint: &Paint) {
        unsafe { self.as_mut().drawSimpleText(text.as_ptr().cast(), text.len(), sys::SkTextEncoding_kUTF8 as sys::SkTextEncoding, x, y, font.0, &*paint.0) };
    }

    pub fn save_count(&self) -> i32 {
        unsafe { sys::SkCanvas_getSaveCount(self.ptr) }
    }

    pub fn rotate_pivot(&mut self, degrees: f32, px: f32, py: f32) {
        unsafe { self.as_mut().rotate1(degrees, px, py) };
    }

    pub fn read_pixels_bitmap(&self, bitmap: &mut Bitmap, src_x: i32, src_y: i32) -> bool {
        unsafe { sys::SkCanvas_readPixels2(self.ptr, bitmap.as_raw_mut(), src_x, src_y) }
    }

    pub fn write_pixels_bitmap(&mut self, bitmap: &Bitmap, x: i32, y: i32) -> bool {
        unsafe { sys::SkCanvas_writePixels1(self.ptr, bitmap.as_raw(), x, y) }
    }

    pub fn draw_picture_with_matrix(&mut self, picture: &Picture, matrix: Option<&Matrix>, paint: Option<&Paint>) {
        let matrix_ptr = matrix.map_or(std::ptr::null(), |m| &m.0 as *const sys::SkMatrix);
        let paint_ptr = paint.map_or(std::ptr::null(), |p| &*p.0 as *const sys::SkPaint);
        unsafe { sys::SkCanvas_drawPicture2(self.ptr, picture.0, matrix_ptr, paint_ptr) };
    }

    pub fn draw_drawable_at(&mut self, drawable: &crate::Drawable, x: f32, y: f32) {
        unsafe { self.as_mut().drawDrawable1(drawable.as_raw(), x, y) };
    }

    pub fn local_to_device(&self) -> M44 {
        unsafe { M44::from_raw(sys::skialin_bridge_Canvas_getLocalToDevice(self.ptr)).expect("getLocalToDevice never returns null") }
    }

    pub fn clip_rect(&mut self, rect: Rect, op: ClipOp, antialias: bool) {
        let sk_rect: sys::SkRect = rect.into();
        unsafe { self.as_mut().clipRect(&sk_rect, op.into(), antialias) };
    }

    pub fn clip_path(&mut self, path: &Path, op: ClipOp, antialias: bool) {
        unsafe { self.as_mut().clipPath(path.0, op.into(), antialias) };
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

    pub fn draw_image_nine(&mut self, image: &Image, center: IRect, dst: Rect, filter: FilterMode, paint: Option<&Paint>) {
        let sk_center: sys::SkIRect = center.into();
        let sk_dst: sys::SkRect = dst.into();
        let paint_ptr = paint.map_or(std::ptr::null(), |p| &*p.0 as *const sys::SkPaint);
        unsafe { self.as_mut().drawImageNine(image.0, &sk_center, &sk_dst, filter.into(), paint_ptr) };
    }

    pub fn save_layer(&mut self, bounds: Option<Rect>, paint: Option<&Paint>) -> i32 {
        let sk_bounds: Option<sys::SkRect> = bounds.map(Into::into);
        let bounds_ptr = sk_bounds.as_ref().map_or(std::ptr::null(), |r| r as *const sys::SkRect);
        let paint_ptr = paint.map_or(std::ptr::null(), |p| &*p.0 as *const sys::SkPaint);
        unsafe { self.as_mut().saveLayer(bounds_ptr, paint_ptr) }
    }

    pub fn save_layer_with_backdrop(&mut self, bounds: Option<Rect>, paint: Option<&Paint>, backdrop: Option<&crate::ImageFilter>, flags: u32) -> i32 {
        let sk_bounds: Option<sys::SkRect> = bounds.map(Into::into);
        let bounds_ptr = sk_bounds.as_ref().map_or(std::ptr::null(), |r| r as *const sys::SkRect);
        let paint_ptr = paint.map_or(std::ptr::null(), |p| &*p.0 as *const sys::SkPaint);
        let backdrop_ptr = backdrop.map_or(std::ptr::null_mut(), |f| f.0);
        unsafe { sys::skialin_bridge_Canvas_saveLayer(self.ptr, bounds_ptr, paint_ptr, backdrop_ptr, flags) }
    }

    pub fn draw_rrect(&mut self, rrect: &RRect, paint: &Paint) {
        unsafe { sys::skialin_bridge_Canvas_drawRRect(self.ptr, rrect.0, &*paint.0) };
    }

    pub fn draw_picture(&mut self, picture: &crate::Picture) {
        unsafe { sys::skialin_bridge_Canvas_drawPicture(self.ptr, picture.0) };
    }

    pub fn draw_drrect(&mut self, outer: &RRect, inner: &RRect, paint: &Paint) {
        unsafe { sys::skialin_bridge_Canvas_drawDRRect(self.ptr, outer.0, inner.0, &*paint.0) };
    }

    pub fn clip_rrect(&mut self, rrect: &RRect, op: ClipOp, antialias: bool) {
        unsafe { sys::skialin_bridge_Canvas_clipRRect(self.ptr, rrect.0, op.into(), antialias) };
    }

    pub fn draw_region(&mut self, region: &Region, paint: &Paint) {
        unsafe { self.as_mut().drawRegion(region.0, &*paint.0) };
    }

    pub fn clip_region(&mut self, region: &Region, op: ClipOp) {
        unsafe { self.as_mut().clipRegion(region.0, op.into()) };
    }

    pub fn draw_vertices(&mut self, vertices: &Vertices, mode: BlendMode, paint: &Paint) {
        unsafe { sys::skialin_bridge_Canvas_drawVertices(self.ptr, vertices.0, mode.into(), &*paint.0) };
    }

    pub fn draw_patch(&mut self, cubics: [Point; 12], colors: [Color; 4], tex_coords: Option<[Point; 4]>, mode: BlendMode, paint: &Paint) {
        let sk_cubics: Vec<sys::SkPoint> = cubics.iter().map(|&p| p.into()).collect();
        let sk_tex: Option<[sys::SkPoint; 4]> = tex_coords.map(|coords| std::array::from_fn(|i| coords[i].into()));
        let tex_ptr = sk_tex.as_ref().map_or(std::ptr::null(), |t| t.as_ptr());
        unsafe { self.as_mut().drawPatch(sk_cubics.as_ptr(), colors.as_ptr(), tex_ptr, mode.into(), &*paint.0) };
    }

    pub fn draw_annotation(&mut self, rect: Rect, key: &str, value: Option<&Data>) {
        let sk_rect: sys::SkRect = rect.into();
        let key_cstr = std::ffi::CString::new(key).expect("annotation key must not contain a NUL byte");
        let value_ptr = value.map_or(std::ptr::null_mut(), |d| d.0);
        unsafe { self.as_mut().drawAnnotation(&sk_rect, key_cstr.as_ptr(), value_ptr) };
    }

    pub fn draw_drawable(&mut self, drawable: &crate::Drawable, matrix: Option<&Matrix>) {
        let matrix_ptr = matrix.map_or(std::ptr::null(), |m| &m.0 as *const sys::SkMatrix);
        unsafe { self.as_mut().drawDrawable(drawable.as_raw(), matrix_ptr) };
    }

    pub fn concat_44(&mut self, matrix: &M44) {
        unsafe { sys::skialin_bridge_Canvas_concat44(self.ptr, matrix.0) };
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw_shadow(
        &mut self,
        path: &Path,
        z_plane: (f32, f32, f32),
        light_pos: (f32, f32, f32),
        light_radius: f32,
        ambient_color: Color,
        spot_color: Color,
        flags: u32,
    ) {
        unsafe {
            sys::skialin_bridge_ShadowUtils_drawShadow(
                self.ptr,
                path.0,
                z_plane.0,
                z_plane.1,
                z_plane.2,
                light_pos.0,
                light_pos.1,
                light_pos.2,
                light_radius,
                ambient_color,
                spot_color,
                flags,
            )
        };
    }
}
