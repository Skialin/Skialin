use crate::{sys, Blender, Color, Matrix, Point, TileMode};

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

    pub fn linear_gradient(pts: [Point; 2], colors: &[Color], positions: Option<&[f32]>, tile_mode: TileMode, local_matrix: Option<&Matrix>) -> Option<Self> {
        let sk_pts = [sys::SkPoint::from(pts[0]), sys::SkPoint::from(pts[1])];
        let (pos_ptr, count) = positions_ptr(positions, colors.len());
        let matrix_ptr = local_matrix.map_or(std::ptr::null(), |m| &m.0);
        unsafe { Self::from_raw(sys::skialin_bridge_Shader_makeLinearGradient(sk_pts.as_ptr(), colors.as_ptr(), pos_ptr, count, tile_mode.into(), matrix_ptr)) }
    }

    pub fn radial_gradient(center: Point, radius: f32, colors: &[Color], positions: Option<&[f32]>, tile_mode: TileMode, local_matrix: Option<&Matrix>) -> Option<Self> {
        let (pos_ptr, count) = positions_ptr(positions, colors.len());
        let matrix_ptr = local_matrix.map_or(std::ptr::null(), |m| &m.0);
        unsafe { Self::from_raw(sys::skialin_bridge_Shader_makeRadialGradient(center.into(), radius, colors.as_ptr(), pos_ptr, count, tile_mode.into(), matrix_ptr)) }
    }

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

    pub fn sweep_gradient(center: Point, start_angle: f32, end_angle: f32, colors: &[Color], positions: Option<&[f32]>, tile_mode: TileMode, local_matrix: Option<&Matrix>) -> Option<Self> {
        let (pos_ptr, count) = positions_ptr(positions, colors.len());
        let matrix_ptr = local_matrix.map_or(std::ptr::null(), |m| &m.0);
        unsafe { Self::from_raw(sys::skialin_bridge_Shader_makeSweepGradient(center.into(), start_angle, end_angle, colors.as_ptr(), pos_ptr, count, tile_mode.into(), matrix_ptr)) }
    }

    pub fn blend(mode: crate::BlendMode, dst: &Shader, src: &Shader) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_Shader_Blend(mode.into(), dst.0, src.0)) }
    }

    pub fn blend_with_blender(blender: &Blender, dst: &Shader, src: &Shader) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_Shader_BlendBlender(blender.0, dst.0, src.0)) }
    }

    pub fn fractal_noise(base_freq_x: f32, base_freq_y: f32, num_octaves: i32, seed: f32) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_Shader_MakeFractalNoise(base_freq_x, base_freq_y, num_octaves, seed)) }
    }

    pub fn turbulence(base_freq_x: f32, base_freq_y: f32, num_octaves: i32, seed: f32) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_Shader_MakeTurbulence(base_freq_x, base_freq_y, num_octaves, seed)) }
    }
}

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
