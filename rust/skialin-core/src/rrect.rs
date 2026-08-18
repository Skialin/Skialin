use crate::{sys, Matrix, Point, Rect};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RRectType {
    Empty,
    Rect,
    Oval,
    Simple,
    NinePatch,
    Complex,
}

impl From<i32> for RRectType {
    fn from(value: i32) -> Self {
        match value {
            1 => RRectType::Rect,
            2 => RRectType::Oval,
            3 => RRectType::Simple,
            4 => RRectType::NinePatch,
            5 => RRectType::Complex,
            _ => RRectType::Empty,
        }
    }
}

/// The four corner radii, in `[upper_left, upper_right, lower_right, lower_left]` order.
pub type CornerRadii = [(f32, f32); 4];

pub struct RRect(pub(crate) *mut sys::SkRRect);

impl RRect {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::SkRRect) -> Option<Self> {
        (!ptr.is_null()).then_some(RRect(ptr))
    }

    pub fn make_rect(rect: Rect) -> Self {
        let sk_rect: sys::SkRect = rect.into();
        RRect(unsafe { sys::skialin_bridge_RRect_MakeRect(&sk_rect) })
    }

    pub fn make_oval(oval: Rect) -> Self {
        let sk_rect: sys::SkRect = oval.into();
        RRect(unsafe { sys::skialin_bridge_RRect_MakeOval(&sk_rect) })
    }

    pub fn make_rect_xy(rect: Rect, x_rad: f32, y_rad: f32) -> Self {
        let sk_rect: sys::SkRect = rect.into();
        RRect(unsafe { sys::skialin_bridge_RRect_MakeRectXY(&sk_rect, x_rad, y_rad) })
    }

    pub fn make_rect_radii(rect: Rect, radii: CornerRadii) -> Self {
        let sk_rect: sys::SkRect = rect.into();
        let flat: [f32; 8] = [radii[0].0, radii[0].1, radii[1].0, radii[1].1, radii[2].0, radii[2].1, radii[3].0, radii[3].1];
        RRect(unsafe { sys::skialin_bridge_RRect_MakeRectRadii(&sk_rect, flat.as_ptr()) })
    }

    pub fn rect(&self) -> Rect {
        let mut out = sys::SkRect::default();
        unsafe { sys::skialin_bridge_RRect_rect(self.0, &mut out) };
        out.into()
    }

    pub fn radii(&self) -> CornerRadii {
        let mut flat = [0f32; 8];
        unsafe { sys::skialin_bridge_RRect_radii(self.0, flat.as_mut_ptr()) };
        [(flat[0], flat[1]), (flat[2], flat[3]), (flat[4], flat[5]), (flat[6], flat[7])]
    }

    pub fn rrect_type(&self) -> RRectType {
        unsafe { sys::skialin_bridge_RRect_type(self.0) }.into()
    }

    pub fn is_empty(&self) -> bool {
        self.rrect_type() == RRectType::Empty
    }

    pub fn contains_point(&self, point: Point) -> bool {
        unsafe { sys::skialin_bridge_RRect_containsPoint(self.0, point.into()) }
    }

    pub fn contains_rect(&self, rect: Rect) -> bool {
        let sk_rect: sys::SkRect = rect.into();
        unsafe { sys::skialin_bridge_RRect_containsRect(self.0, &sk_rect) }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { sys::skialin_bridge_RRect_isValid(self.0) }
    }

    pub fn inset(&self, dx: f32, dy: f32) -> RRect {
        RRect(unsafe { sys::skialin_bridge_RRect_inset(self.0, dx, dy) })
    }

    pub fn outset(&self, dx: f32, dy: f32) -> RRect {
        RRect(unsafe { sys::skialin_bridge_RRect_outset(self.0, dx, dy) })
    }

    /// `None` if `matrix` doesn't preserve the rounded-rect shape (e.g. a skew).
    pub fn transform(&self, matrix: &Matrix) -> Option<RRect> {
        unsafe { Self::from_raw(sys::skialin_bridge_RRect_transform(self.0, &matrix.0)) }
    }
}

impl Clone for RRect {
    fn clone(&self) -> Self {
        RRect(unsafe { sys::skialin_bridge_RRect_clone(self.0) })
    }
}

impl Drop for RRect {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_RRect_delete(self.0) };
    }
}
