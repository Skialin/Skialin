use crate::{sys, ColorFilter, Matrix, Shader};

pub struct RuntimeEffect(pub(crate) *mut sys::SkRuntimeEffect);

impl RuntimeEffect {
    /// Compiles an SkSL shader effect. The SkSL must define
    /// `vec4 main(vec2 coord) { ... }` returning a premultiplied color.
    /// On failure, returns the compiler's error text.
    pub fn make_for_shader(sksl: &str) -> Result<Self, String> {
        let mut error: *mut sys::SkData = std::ptr::null_mut();
        let effect = unsafe { sys::skialin_bridge_RuntimeEffect_MakeForShader(sksl.as_ptr().cast(), sksl.len(), &mut error) };
        Self::from_result(effect, error)
    }

    /// Compiles an SkSL color-filter effect. The SkSL must define
    /// `vec4 main(vec4 inColor) { ... }`. On failure, returns the
    /// compiler's error text.
    pub fn make_for_color_filter(sksl: &str) -> Result<Self, String> {
        let mut error: *mut sys::SkData = std::ptr::null_mut();
        let effect = unsafe { sys::skialin_bridge_RuntimeEffect_MakeForColorFilter(sksl.as_ptr().cast(), sksl.len(), &mut error) };
        Self::from_result(effect, error)
    }

    fn from_result(effect: *mut sys::SkRuntimeEffect, error: *mut sys::SkData) -> Result<Self, String> {
        if !effect.is_null() {
            return Ok(RuntimeEffect(effect));
        }
        let message = unsafe { crate::Data::from_raw(error) }.map(|d| String::from_utf8_lossy(d.as_bytes()).into_owned()).unwrap_or_default();
        Err(message)
    }

    /// `uniforms` is a raw byte buffer packed to match the SkSL uniform
    /// block's layout (the caller is responsible for knowing that layout).
    pub fn make_shader(&self, uniforms: &[u8], children: &[&Shader], local_matrix: Option<&Matrix>) -> Option<Shader> {
        let child_ptrs: Vec<*mut sys::SkShader> = children.iter().map(|c| c.0).collect();
        let matrix_ptr = local_matrix.map_or(std::ptr::null(), |m| &m.0);
        unsafe {
            Shader::from_raw(sys::skialin_bridge_RuntimeEffect_makeShader(
                self.0,
                uniforms.as_ptr(),
                uniforms.len(),
                child_ptrs.as_ptr(),
                child_ptrs.len(),
                matrix_ptr,
            ))
        }
    }

    /// `uniforms` is a raw byte buffer packed to match the SkSL uniform
    /// block's layout (the caller is responsible for knowing that layout).
    pub fn make_color_filter(&self, uniforms: &[u8], children: &[&ColorFilter]) -> Option<ColorFilter> {
        let child_ptrs: Vec<*mut sys::SkColorFilter> = children.iter().map(|c| c.0).collect();
        unsafe { ColorFilter::from_raw(sys::skialin_bridge_RuntimeEffect_makeColorFilter(self.0, uniforms.as_ptr(), uniforms.len(), child_ptrs.as_ptr(), child_ptrs.len())) }
    }
}

impl Drop for RuntimeEffect {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_RuntimeEffect_unref(self.0) };
    }
}
