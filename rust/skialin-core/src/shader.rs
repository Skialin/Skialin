use crate::{sys, Color, Matrix};

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
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Shader_unref(self.0) };
    }
}
