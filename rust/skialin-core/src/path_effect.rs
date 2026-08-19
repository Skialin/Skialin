use crate::{sys, Matrix, Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Path1DStyle {
    Translate,
    Rotate,
    Morph,
}

impl From<Path1DStyle> for i32 {
    fn from(style: Path1DStyle) -> Self {
        match style {
            Path1DStyle::Translate => 0,
            Path1DStyle::Rotate => 1,
            Path1DStyle::Morph => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrimMode {
    /// Keeps the `[start_t, stop_t]` subset.
    Normal,
    /// Keeps the complement: `[0, start_t] + [stop_t, 1]`.
    Inverted,
}

impl From<TrimMode> for i32 {
    fn from(mode: TrimMode) -> Self {
        match mode {
            TrimMode::Normal => 0,
            TrimMode::Inverted => 1,
        }
    }
}

pub struct PathEffect(pub(crate) *mut sys::SkPathEffect);

impl PathEffect {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::SkPathEffect) -> Option<Self> {
        (!ptr.is_null()).then_some(PathEffect(ptr))
    }

    /// `None` if `intervals` is empty, has an odd length, or any interval is negative.
    pub fn dash(intervals: &[f32], phase: f32) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_PathEffect_MakeDash(intervals.as_ptr(), intervals.len(), phase)) }
    }

    /// Rounds each corner of the path to the given radius.
    pub fn corner(radius: f32) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_PathEffect_MakeCorner(radius)) }
    }

    /// Roughens the path by displacing points along it.
    pub fn discrete(seg_length: f32, deviation: f32, seed_assist: u32) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_PathEffect_MakeDiscrete(seg_length, deviation, seed_assist)) }
    }

    /// Keeps only a `[start_t, stop_t]` subset of the path (or its complement in `Inverted` mode).
    pub fn trim(start_t: f32, stop_t: f32, mode: TrimMode) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_PathEffect_MakeTrim(start_t, stop_t, mode.into())) }
    }

    /// `result = outer(inner(path))`.
    pub fn compose(outer: &PathEffect, inner: &PathEffect) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_PathEffect_MakeCompose(outer.0, inner.0)) }
    }

    /// Applies `first` and `second` independently, then draws both results.
    pub fn sum(first: &PathEffect, second: &PathEffect) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_PathEffect_MakeSum(first.0, second.0)) }
    }

    pub fn path_1d(path: &Path, advance: f32, phase: f32, style: Path1DStyle) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_PathEffect_MakePath1D(path.0, advance, phase, style.into())) }
    }

    pub fn path_2d(matrix: &Matrix, path: &Path) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_PathEffect_MakePath2D(&matrix.0, path.0)) }
    }

    pub fn line_2d(width: f32, matrix: &Matrix) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_PathEffect_MakeLine2D(width, &matrix.0)) }
    }
}

impl Drop for PathEffect {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_PathEffect_unref(self.0) };
    }
}
