use std::marker::PhantomData;

use crate::paint::BlendMode;
use crate::path::Path;
use crate::{sys, Color, Matrix, Paint, Point, Rect};

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
        unsafe { self.as_mut().drawPaint(&paint.0) };
    }

    pub fn draw_line(&mut self, p0: Point, p1: Point, paint: &Paint) {
        unsafe { self.as_mut().drawLine1(p0.into(), p1.into(), &paint.0) };
    }

    pub fn draw_rect(&mut self, rect: Rect, paint: &Paint) {
        let sk_rect: sys::SkRect = rect.into();
        unsafe { self.as_mut().drawRect(&sk_rect, &paint.0) };
    }

    pub fn draw_oval(&mut self, rect: Rect, paint: &Paint) {
        let sk_rect: sys::SkRect = rect.into();
        unsafe { self.as_mut().drawOval(&sk_rect, &paint.0) };
    }

    pub fn draw_circle(&mut self, center: Point, radius: f32, paint: &Paint) {
        unsafe { self.as_mut().drawCircle1(center.into(), radius, &paint.0) };
    }

    pub fn draw_path(&mut self, path: &Path, paint: &Paint) {
        unsafe { self.as_mut().drawPath(&path.0, &paint.0) };
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
        unsafe { self.as_mut().clipPath1(&path.0, op.into()) };
    }
}
