use crate::{sys, Color, Matrix, Point, TileMode};

/// Specifies the source color(s) for a [`crate::Paint`]. Mirrors Skia's
/// `SkShader`. Just the base handle plus the factories that don't need a
/// full image/gradient/effect API yet.
pub struct Shader(pub(crate) *mut sys::SkShader);

impl Shader {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::SkShader) -> Option<Self> {
        (!ptr.is_null()).then_some(Shader(ptr))
    }

    pub fn empty() -> Self {
        Shader(unsafe { sys::skialin_bridge_Shader_makeEmpty() })
    }

    pub fn color(color: Color) -> Self {
        Shader(unsafe { sys::skialin_bridge_Shader_makeColor(color) })
    }

    pub fn is_opaque(&self) -> bool {
        unsafe { sys::skialin_bridge_Shader_isOpaque(self.0) }
    }

    pub fn with_local_matrix(&self, matrix: &Matrix) -> Shader {
        Shader(unsafe { sys::skialin_bridge_Shader_makeWithLocalMatrix(self.0, &matrix.0) })
    }

    /// A gradient between `pts[0]` and `pts[1]`. `positions`, if given, must
    /// have the same length as `colors`: strictly increasing values in
    /// `[0, 1]`. `None` if `colors.len() < 2` or `positions` is malformed.
    pub fn linear_gradient(pts: [Point; 2], colors: &[Color], positions: Option<&[f32]>, tile_mode: TileMode, local_matrix: Option<&Matrix>) -> Option<Self> {
        let sk_pts = [sys::SkPoint::from(pts[0]), sys::SkPoint::from(pts[1])];
        let (pos_ptr, count) = positions_ptr(positions, colors.len());
        let matrix_ptr = local_matrix.map_or(std::ptr::null(), |m| &m.0);
        unsafe { Self::from_raw(sys::skialin_bridge_Shader_makeLinearGradient(sk_pts.as_ptr(), colors.as_ptr(), pos_ptr, count, tile_mode.into(), matrix_ptr)) }
    }

    /// A gradient radiating from `center` out to `radius`. `radius` must be
    /// positive. `None` if `colors.len() < 2`, `radius <= 0`, or
    /// `positions` is malformed.
    pub fn radial_gradient(center: Point, radius: f32, colors: &[Color], positions: Option<&[f32]>, tile_mode: TileMode, local_matrix: Option<&Matrix>) -> Option<Self> {
        let (pos_ptr, count) = positions_ptr(positions, colors.len());
        let matrix_ptr = local_matrix.map_or(std::ptr::null(), |m| &m.0);
        unsafe { Self::from_raw(sys::skialin_bridge_Shader_makeRadialGradient(center.into(), radius, colors.as_ptr(), pos_ptr, count, tile_mode.into(), matrix_ptr)) }
    }

    /// A gradient between two circles; both radii must be non-negative.
    /// `None` if the inputs are invalid.
    #[allow(clippy::too_many_arguments)]
    pub fn two_point_conical_gradient(
        start: Point,
        start_radius: f32,
        end: Point,
        end_radius: f32,
        colors: &[Color],
        positions: Option<&[f32]>,
        tile_mode: TileMode,
        local_matrix: Option<&Matrix>,
    ) -> Option<Self> {
        let (pos_ptr, count) = positions_ptr(positions, colors.len());
        let matrix_ptr = local_matrix.map_or(std::ptr::null(), |m| &m.0);
        unsafe {
            Self::from_raw(sys::skialin_bridge_Shader_makeTwoPointConicalGradient(
                start.into(),
                start_radius,
                end.into(),
                end_radius,
                colors.as_ptr(),
                pos_ptr,
                count,
                tile_mode.into(),
                matrix_ptr,
            ))
        }
    }

    /// A gradient sweeping around `center` from `start_angle` to
    /// `end_angle` degrees (0 = positive x axis). `start_angle` must be
    /// less than `end_angle`.
    pub fn sweep_gradient(center: Point, start_angle: f32, end_angle: f32, colors: &[Color], positions: Option<&[f32]>, tile_mode: TileMode, local_matrix: Option<&Matrix>) -> Option<Self> {
        let (pos_ptr, count) = positions_ptr(positions, colors.len());
        let matrix_ptr = local_matrix.map_or(std::ptr::null(), |m| &m.0);
        unsafe { Self::from_raw(sys::skialin_bridge_Shader_makeSweepGradient(center.into(), start_angle, end_angle, colors.as_ptr(), pos_ptr, count, tile_mode.into(), matrix_ptr)) }
    }
}

/// `positions`, if given, must have the same length as the color array.
fn positions_ptr(positions: Option<&[f32]>, color_count: usize) -> (*const f32, usize) {
    match positions {
        Some(p) => {
            assert_eq!(p.len(), color_count, "gradient positions.len() must equal colors.len()");
            (p.as_ptr(), color_count)
        }
        None => (std::ptr::null(), color_count),
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Shader_unref(self.0) };
    }
}
