use crate::{sys, Matrix, Point, Rect};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PathDirection {
    Clockwise,
    CounterClockwise,
}

impl From<PathDirection> for sys::SkPathDirection {
    fn from(direction: PathDirection) -> Self {
        match direction {
            PathDirection::Clockwise => sys::SkPathDirection_kCW,
            PathDirection::CounterClockwise => sys::SkPathDirection_kCCW,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathOp {
    Difference,
    Intersect,
    Union,
    Xor,
    ReverseDifference,
}

impl From<PathOp> for i32 {
    fn from(op: PathOp) -> Self {
        match op {
            PathOp::Difference => 0,
            PathOp::Intersect => 1,
            PathOp::Union => 2,
            PathOp::Xor => 3,
            PathOp::ReverseDifference => 4,
        }
    }
}

/// An immutable, drawable path snapshot. Produced from a [`PathBuilder`].
pub struct Path(pub(crate) *mut sys::SkPath);

impl Path {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::SkPath) -> Option<Self> {
        (!ptr.is_null()).then_some(Path(ptr))
    }

    pub fn is_empty(&self) -> bool {
        unsafe { (*self.0).isEmpty() }
    }

    pub fn bounds(&self) -> Rect {
        unsafe { *(*self.0).getBounds() }.into()
    }

    pub fn contains(&self, point: Point) -> bool {
        unsafe { (*self.0).contains(point.into()) }
    }

    /// Combines `one` and `two` with the given boolean operation. `None` if
    /// the operation couldn't produce a result.
    pub fn op(one: &Path, two: &Path, op: PathOp) -> Option<Path> {
        unsafe { Self::from_raw(sys::skialin_bridge_Path_op(one.0, two.0, op.into())) }
    }

    /// A path with the same non-overlapping-contour area as this one,
    /// with self-intersections removed. `None` on failure.
    pub fn simplify(&self) -> Option<Path> {
        unsafe { Self::from_raw(sys::skialin_bridge_Path_simplify(self.0)) }
    }
}

impl Clone for Path {
    fn clone(&self) -> Self {
        Path(unsafe { sys::skialin_bridge_Path_clone(self.0) })
    }
}

impl Drop for Path {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Path_delete(self.0) };
    }
}

/// Mutable path construction, mirroring Skia's `SkPathBuilder`. Call
/// [`PathBuilder::snapshot`] to obtain a drawable [`Path`] without
/// consuming the builder, or [`PathBuilder::detach`] to take it and reset.
pub struct PathBuilder(Box<sys::SkPathBuilder>);

impl PathBuilder {
    pub fn new() -> Self {
        PathBuilder(crate::support::new_boxed(sys::SkPathBuilder_SkPathBuilder))
    }

    pub(crate) fn as_raw_mut(&mut self) -> *mut sys::SkPathBuilder {
        &mut *self.0
    }

    pub fn move_to(&mut self, point: Point) -> &mut Self {
        unsafe { self.0.moveTo(point.into()) };
        self
    }

    pub fn line_to(&mut self, point: Point) -> &mut Self {
        unsafe { self.0.lineTo(point.into()) };
        self
    }

    pub fn quad_to(&mut self, p1: Point, p2: Point) -> &mut Self {
        unsafe { self.0.quadTo(p1.into(), p2.into()) };
        self
    }

    pub fn cubic_to(&mut self, p1: Point, p2: Point, p3: Point) -> &mut Self {
        unsafe { self.0.cubicTo(p1.into(), p2.into(), p3.into()) };
        self
    }

    pub fn arc_to(&mut self, oval: Rect, start_angle_deg: f32, sweep_angle_deg: f32, force_move_to: bool) -> &mut Self {
        let sk_rect: sys::SkRect = oval.into();
        unsafe { self.0.arcTo(&sk_rect, start_angle_deg, sweep_angle_deg, force_move_to) };
        self
    }

    pub fn close(&mut self) -> &mut Self {
        unsafe { self.0.close() };
        self
    }

    pub fn add_rect(&mut self, rect: Rect, direction: PathDirection) -> &mut Self {
        let sk_rect: sys::SkRect = rect.into();
        unsafe { self.0.addRect(&sk_rect, direction.into(), 0) };
        self
    }

    pub fn add_oval(&mut self, oval: Rect, direction: PathDirection) -> &mut Self {
        let sk_rect: sys::SkRect = oval.into();
        unsafe { self.0.addOval(&sk_rect, direction.into(), 1) };
        self
    }

    pub fn add_circle(&mut self, center: Point, radius: f32, direction: PathDirection) -> &mut Self {
        unsafe { self.0.addCircle(center.into(), radius, direction.into()) };
        self
    }

    pub fn offset(&mut self, dx: f32, dy: f32) -> &mut Self {
        unsafe { self.0.offset(dx, dy) };
        self
    }

    pub fn is_empty(&self) -> bool {
        unsafe { self.0.isEmpty() }
    }

    pub fn bounds(&self) -> Rect {
        unsafe { sys::skialin_bridge_PathBuilder_computeBounds(&*self.0) }.into()
    }

    /// Snapshots the current contents into a drawable [`Path`] without
    /// consuming the builder.
    pub fn snapshot(&self) -> Path {
        Path(unsafe { sys::skialin_bridge_PathBuilder_snapshot(&*self.0, std::ptr::null()) })
    }

    /// Snapshots and applies `matrix`, without consuming the builder.
    pub fn snapshot_with_matrix(&self, matrix: &Matrix) -> Path {
        Path(unsafe { sys::skialin_bridge_PathBuilder_snapshot(&*self.0, &matrix.0) })
    }

    /// Takes the current contents into a drawable [`Path`], resetting the
    /// builder to empty.
    pub fn detach(&mut self) -> Path {
        Path(unsafe { sys::skialin_bridge_PathBuilder_detach(&mut *self.0, std::ptr::null()) })
    }
}

impl Default for PathBuilder {
    fn default() -> Self {
        PathBuilder::new()
    }
}

impl Drop for PathBuilder {
    fn drop(&mut self) {
        unsafe { self.0.destruct() };
    }
}
