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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PathFillType {
    Winding,
    EvenOdd,
    InverseWinding,
    InverseEvenOdd,
}

impl From<sys::SkPathFillType> for PathFillType {
    fn from(fill_type: sys::SkPathFillType) -> Self {
        match fill_type {
            sys::SkPathFillType_kEvenOdd => PathFillType::EvenOdd,
            sys::SkPathFillType_kInverseWinding => PathFillType::InverseWinding,
            sys::SkPathFillType_kInverseEvenOdd => PathFillType::InverseEvenOdd,
            _ => PathFillType::Winding,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AddPathMode {
    Append,
    Extend,
}

impl From<AddPathMode> for sys::SkPath_AddPathMode {
    fn from(mode: AddPathMode) -> Self {
        (match mode {
            AddPathMode::Append => sys::SkPath_AddPathMode_kAppend_AddPathMode,
            AddPathMode::Extend => sys::SkPath_AddPathMode_kExtend_AddPathMode,
        }) as sys::SkPath_AddPathMode
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

    pub fn fill_type(&self) -> crate::PathFillType {
        unsafe { (*self.0).getFillType() }.into()
    }

    pub fn is_convex(&self) -> bool {
        unsafe { (*self.0).isConvex() }
    }

    pub fn is_oval(&self) -> Option<Rect> {
        let mut bounds = sys::SkRect::default();
        let found = unsafe { (*self.0).isOval(&mut bounds) };
        found.then(|| bounds.into())
    }

    pub fn is_rrect(&self) -> Option<crate::RRect> {
        let scratch = crate::RRect::make_rect(Rect::new(0.0, 0.0, 0.0, 0.0));
        let found = unsafe { (*self.0).isRRect(scratch.0) };
        found.then_some(scratch)
    }

    pub fn compute_tight_bounds(&self) -> Rect {
        let mut out = sys::SkRect::default();
        unsafe { sys::skialin_bridge_Path_computeTightBounds(self.0, &mut out) };
        out.into()
    }

    fn get_points(&self, buf: &mut [sys::SkPoint]) -> usize {
        let span = sys::SkSpan { _phantom_0: std::marker::PhantomData, fPtr: buf.as_mut_ptr(), fSize: buf.len() };
        unsafe { sys::SkPath_getPoints(self.0, span) as usize }
    }

    pub fn points_count(&self) -> i32 {
        self.get_points(&mut []) as i32
    }

    pub fn points(&self) -> Vec<Point> {
        let total = self.get_points(&mut []);
        let mut buf = vec![sys::SkPoint::default(); total];
        self.get_points(&mut buf);
        buf.into_iter().map(Into::into).collect()
    }

    pub fn generation_id(&self) -> u32 {
        unsafe { (*self.0).getGenerationID() }
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

    pub fn r_move_to(&mut self, dx: f32, dy: f32) -> &mut Self {
        unsafe { self.0.rMoveTo(Point::new(dx, dy).into()) };
        self
    }

    pub fn r_line_to(&mut self, dx: f32, dy: f32) -> &mut Self {
        unsafe { self.0.rLineTo(Point::new(dx, dy).into()) };
        self
    }

    pub fn r_quad_to(&mut self, dx1: f32, dy1: f32, dx2: f32, dy2: f32) -> &mut Self {
        unsafe { self.0.rQuadTo(Point::new(dx1, dy1).into(), Point::new(dx2, dy2).into()) };
        self
    }

    pub fn r_cubic_to(&mut self, dx1: f32, dy1: f32, dx2: f32, dy2: f32, dx3: f32, dy3: f32) -> &mut Self {
        unsafe { self.0.rCubicTo(Point::new(dx1, dy1).into(), Point::new(dx2, dy2).into(), Point::new(dx3, dy3).into()) };
        self
    }

    pub fn conic_to(&mut self, p1: Point, p2: Point, w: f32) -> &mut Self {
        unsafe { self.0.conicTo(p1.into(), p2.into(), w) };
        self
    }

    pub fn r_conic_to(&mut self, dx1: f32, dy1: f32, dx2: f32, dy2: f32, w: f32) -> &mut Self {
        unsafe { self.0.rConicTo(Point::new(dx1, dy1).into(), Point::new(dx2, dy2).into(), w) };
        self
    }

    pub fn add_rrect(&mut self, rrect: &crate::RRect, direction: PathDirection) -> &mut Self {
        unsafe { self.0.addRRect(rrect.0, direction.into(), 0) };
        self
    }

    pub fn add_poly(&mut self, points: &[Point], close: bool) -> &mut Self {
        let sk_points: Vec<sys::SkPoint> = points.iter().map(|&p| p.into()).collect();
        let span = sys::SkSpan { _phantom_0: std::marker::PhantomData, fPtr: sk_points.as_ptr().cast_mut(), fSize: sk_points.len() };
        unsafe { self.0.addPolygon(span, close) };
        self
    }

    pub fn add_path(&mut self, src: &Path, dx: f32, dy: f32, mode: AddPathMode) -> &mut Self {
        unsafe { self.0.addPath(src.0, dx, dy, mode.into()) };
        self
    }

    pub fn add_path_matrix(&mut self, src: &Path, matrix: &Matrix, mode: AddPathMode) -> &mut Self {
        unsafe { self.0.addPath2(src.0, &matrix.0, mode.into()) };
        self
    }

    pub fn transform(&mut self, matrix: &Matrix) -> &mut Self {
        unsafe { self.0.transform(&matrix.0) };
        self
    }

    pub fn set_last_pt(&mut self, x: f32, y: f32) -> &mut Self {
        unsafe { self.0.setLastPt(Point::new(x, y).into()) };
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        unsafe { self.0.reset() };
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
