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

    pub fn op(one: &Path, two: &Path, op: PathOp) -> Option<Path> {
        unsafe { Self::from_raw(sys::skialin_bridge_Path_op(one.0, two.0, op.into())) }
    }

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

    /// Walks this path's verbs (move/line/quad/conic/cubic/close), mirroring `SkPath::Iter`.
    ///
    /// When `convert_conics_to_quads` is true, conic segments are approximated with one or more
    /// [`PathVerb::Quad`] segments instead of being reported as [`PathVerb::Conic`]; `tolerance`
    /// controls the maximum deviation of the approximation from the true conic, following the
    /// same tolerance-driven subdivision Skia's own `SkConic::computeQuadPOW2` uses.
    pub fn segments(&self, convert_conics_to_quads: bool, tolerance: f32) -> Vec<PathSegment> {
        let mut iter = unsafe { sys::SkPath_Iter::new1(self.0, false) };
        let mut out = Vec::new();
        loop {
            let mut pts = [sys::SkPoint::default(); 4];
            let verb = unsafe { iter.next(pts.as_mut_ptr()) };
            match verb {
                sys::SkPath_Verb_kMove_Verb => out.push(PathSegment {
                    verb: PathVerb::Move,
                    points: points_from(&pts, 1),
                    conic_weight: 0.0,
                }),
                sys::SkPath_Verb_kLine_Verb => out.push(PathSegment {
                    verb: PathVerb::Line,
                    points: points_from(&pts, 2),
                    conic_weight: 0.0,
                }),
                sys::SkPath_Verb_kQuad_Verb => out.push(PathSegment {
                    verb: PathVerb::Quad,
                    points: points_from(&pts, 3),
                    conic_weight: 0.0,
                }),
                sys::SkPath_Verb_kConic_Verb => {
                    // `SkPath::Iter::conicWeight()` is header-inline (`return *fConicWeights;`)
                    // and isn't exported as a linkable symbol by the prebuilt Skia static lib, so
                    // we read the field directly the same way the inline accessor does. `next()`
                    // has already advanced `fConicWeights` to point at *this* conic's weight.
                    let weight = unsafe { *iter.fConicWeights };
                    if convert_conics_to_quads {
                        push_conic_as_quads(&mut out, &pts, weight, tolerance);
                    } else {
                        out.push(PathSegment { verb: PathVerb::Conic, points: points_from(&pts, 3), conic_weight: weight });
                    }
                }
                sys::SkPath_Verb_kCubic_Verb => out.push(PathSegment {
                    verb: PathVerb::Cubic,
                    points: points_from(&pts, 4),
                    conic_weight: 0.0,
                }),
                sys::SkPath_Verb_kClose_Verb => out.push(PathSegment { verb: PathVerb::Close, points: [Point::new(0.0, 0.0); 4], conic_weight: 0.0 }),
                _ => break,
            }
        }
        out
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PathVerb {
    Move,
    Line,
    Quad,
    Conic,
    Cubic,
    Close,
}

#[derive(Copy, Clone, Debug)]
pub struct PathSegment {
    pub verb: PathVerb,
    /// Meaningful prefix depends on `verb`: Move=1, Line=2, Quad=3, Conic=3, Cubic=4, Close=0.
    pub points: [Point; 4],
    /// Only meaningful when `verb` is [`PathVerb::Conic`].
    pub conic_weight: f32,
}

fn points_from(pts: &[sys::SkPoint; 4], n: usize) -> [Point; 4] {
    let mut out = [Point::new(0.0, 0.0); 4];
    for i in 0..n {
        out[i] = pts[i].into();
    }
    out
}

/// Approximates a conic with 1+ quadratics, mirroring Skia's `SkConic::computeQuadPOW2` tolerance
/// heuristic (the private class isn't exposed to bindgen, so the formula is reimplemented here).
fn conic_to_quad_pow2(p0: sys::SkPoint, p1: sys::SkPoint, p2: sys::SkPoint, w: f32, tol: f32) -> i32 {
    const MAX_POW2: i32 = 5;
    if !(tol >= 0.0) || !tol.is_finite() {
        return 0;
    }
    let a = w - 1.0;
    let k = a / (4.0 * (2.0 + a));
    let x = k * (p0.fX - 2.0 * p1.fX + p2.fX);
    let y = k * (p0.fY - 2.0 * p1.fY + p2.fY);
    let mut error = (x * x + y * y).sqrt();
    let mut pow2 = 0;
    while pow2 < MAX_POW2 {
        if error <= tol {
            break;
        }
        error *= 0.25;
        pow2 += 1;
    }
    pow2
}

fn push_conic_as_quads(out: &mut Vec<PathSegment>, pts: &[sys::SkPoint; 4], w: f32, tol: f32) {
    let (p0, p1, p2) = (pts[0], pts[1], pts[2]);
    let pow2 = conic_to_quad_pow2(p0, p1, p2, w, tol);
    let quad_count = 1usize << pow2;
    let mut buf = vec![sys::SkPoint::default(); 2 * quad_count + 1];
    let n = unsafe { sys::SkPath::ConvertConicToQuads(&p0, &p1, &p2, w, buf.as_mut_ptr(), pow2) };
    for i in 0..n as usize {
        let base = i * 2;
        out.push(PathSegment {
            verb: PathVerb::Quad,
            points: [buf[base].into(), buf[base + 1].into(), buf[base + 2].into(), Point::new(0.0, 0.0)],
            conic_weight: 0.0,
        });
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

pub struct PathBuilder(Box<sys::SkPathBuilder>);

impl PathBuilder {
    pub fn new() -> Self {
        PathBuilder(crate::support::new_boxed(sys::SkPathBuilder_SkPathBuilder))
    }

    /// Seeds a new builder with a copy of `path`'s fill type and verbs, so building can continue
    /// on top of an existing immutable [`Path`] (mirrors `SkPathBuilder(const SkPath&)`).
    pub fn from_path(path: &Path) -> Self {
        let layout = std::alloc::Layout::new::<sys::SkPathBuilder>();
        let ptr = unsafe { std::alloc::alloc(layout) } as *mut sys::SkPathBuilder;
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        unsafe { sys::SkPathBuilder_SkPathBuilder4(ptr, path.0) };
        PathBuilder(unsafe { Box::from_raw(ptr) })
    }

    /// Sets the fill type that will be baked into paths produced by [`Self::snapshot`]/[`Self::detach`].
    pub fn set_fill_type(&mut self, fill_type: PathFillType) -> &mut Self {
        let sk_fill_type: sys::SkPathFillType = match fill_type {
            PathFillType::Winding => sys::SkPathFillType_kWinding,
            PathFillType::EvenOdd => sys::SkPathFillType_kEvenOdd,
            PathFillType::InverseWinding => sys::SkPathFillType_kInverseWinding,
            PathFillType::InverseEvenOdd => sys::SkPathFillType_kInverseEvenOdd,
        };
        unsafe { self.0.setFillType(sk_fill_type) };
        self
    }

    pub fn fill_type(&self) -> PathFillType {
        unsafe { self.0.fillType() }.into()
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

    pub fn snapshot(&self) -> Path {
        Path(unsafe { sys::skialin_bridge_PathBuilder_snapshot(&*self.0, std::ptr::null()) })
    }

    pub fn snapshot_with_matrix(&self, matrix: &Matrix) -> Path {
        Path(unsafe { sys::skialin_bridge_PathBuilder_snapshot(&*self.0, &matrix.0) })
    }

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
