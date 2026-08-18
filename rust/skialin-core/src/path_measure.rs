use crate::{sys, Matrix, Path, PathBuilder, Point};

pub struct PosTan {
    pub position: Point,
    pub tangent: Point,
}

pub struct PathMeasure(*mut sys::SkPathMeasure);

impl PathMeasure {
    pub fn new(path: &Path, force_closed: bool, res_scale: f32) -> Self {
        PathMeasure(unsafe { sys::skialin_bridge_PathMeasure_new(path.0, force_closed, res_scale) })
    }

    /// A path measure with no path attached; call [`Self::set_path`] before using it.
    pub fn empty() -> Self {
        PathMeasure(unsafe { sys::skialin_bridge_PathMeasure_new(std::ptr::null(), false, 1.0) })
    }

    /// `path` of `None` clears the current path.
    pub fn set_path(&mut self, path: Option<&Path>, force_closed: bool) {
        let ptr = path.map_or(std::ptr::null(), |p| p.0 as *const sys::SkPath);
        unsafe { sys::skialin_bridge_PathMeasure_setPath(self.0, ptr, force_closed) };
    }

    /// The length of the current contour, or 0 if there's no path.
    pub fn length(&mut self) -> f32 {
        unsafe { sys::skialin_bridge_PathMeasure_getLength(self.0) }
    }

    /// `distance` is pinned to `[0, length]`. `None` if there's no path or it's zero-length.
    pub fn pos_tan(&mut self, distance: f32) -> Option<PosTan> {
        let mut position = sys::SkPoint::default();
        let mut tangent = sys::SkPoint::default();
        let ok = unsafe { sys::skialin_bridge_PathMeasure_getPosTan(self.0, distance, &mut position, &mut tangent) };
        ok.then_some(PosTan { position: position.into(), tangent: tangent.into() })
    }

    /// `distance` is pinned to `[0, length]`. `None` if there's no path or it's zero-length.
    pub fn matrix(&mut self, distance: f32) -> Option<Matrix> {
        let mut out = Matrix::identity();
        let ok = unsafe { sys::skialin_bridge_PathMeasure_getMatrix(self.0, distance, &mut out.0, 0x3) };
        ok.then_some(out)
    }

    /// Appends the `[start_d, stop_d]` segment of the current contour to `dst`.
    /// `false` (and `dst` untouched) if the segment is zero-length or `start_d > stop_d`.
    pub fn segment(&mut self, start_d: f32, stop_d: f32, dst: &mut PathBuilder, start_with_move_to: bool) -> bool {
        unsafe { sys::skialin_bridge_PathMeasure_getSegment(self.0, start_d, stop_d, dst.as_raw_mut(), start_with_move_to) }
    }

    pub fn is_closed(&mut self) -> bool {
        unsafe { sys::skialin_bridge_PathMeasure_isClosed(self.0) }
    }

    /// Advances to the next contour in the path. `true` if one exists.
    pub fn next_contour(&mut self) -> bool {
        unsafe { sys::skialin_bridge_PathMeasure_nextContour(self.0) }
    }
}

impl Drop for PathMeasure {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_PathMeasure_delete(self.0) };
    }
}
