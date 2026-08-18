use crate::{sys, Point, Rect};

#[derive(Copy, Clone, Debug)]
pub struct Matrix(pub(crate) sys::SkMatrix);

impl Matrix {
    pub fn identity() -> Self {
        let mut m = std::mem::MaybeUninit::<sys::SkMatrix>::uninit();
        unsafe {
            (*m.as_mut_ptr()).setIdentity();
            Matrix(m.assume_init())
        }
    }

    pub fn translate(dx: f32, dy: f32) -> Self {
        let mut m = Self::identity();
        unsafe { m.0.setTranslate(dx, dy) };
        m
    }

    pub fn scale(sx: f32, sy: f32) -> Self {
        let mut m = Self::identity();
        unsafe { m.0.setScale1(sx, sy) };
        m
    }

    pub fn rotate(degrees: f32) -> Self {
        let mut m = Self::identity();
        unsafe { m.0.setRotate1(degrees) };
        m
    }

    pub fn concat(a: &Matrix, b: &Matrix) -> Self {
        let mut m = Self::identity();
        unsafe { m.0.setConcat(&a.0, &b.0) };
        m
    }

    pub fn invert(&self) -> Option<Matrix> {
        let mut out = Self::identity();
        let ok = unsafe { self.0.invert1(&mut out.0) };
        ok.then_some(out)
    }

    pub fn map_point(&self, point: Point) -> Point {
        unsafe { self.0.mapPoint(point.into()).into() }
    }

    pub fn map_rect(&self, rect: Rect) -> Rect {
        let sk_rect: sys::SkRect = rect.into();
        unsafe { self.0.mapRect2(&sk_rect).into() }
    }

    /// Row-major 3x3 matrix values, matching Skia's `get9`/`set9` layout.
    pub fn to_array(&self) -> [f32; 9] {
        let mut out = [0f32; 9];
        unsafe { self.0.get9(out.as_mut_ptr()) };
        out
    }

    pub fn from_array(values: [f32; 9]) -> Self {
        let mut m = Self::identity();
        unsafe { m.0.set9(values.as_ptr()) };
        m
    }
}
