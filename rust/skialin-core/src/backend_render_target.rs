use crate::sys;

/// Wraps a GrBackendRenderTarget: a handle to an existing GPU render target
/// that is not necessarily a sampleable texture (e.g. the window-system
/// framebuffer/FBO 0, or a multisampled renderbuffer), for rendering into it
/// via `Surface::wrap_backend_render_target`.
pub struct BackendRenderTarget(pub(crate) *mut sys::GrBackendRenderTarget);

impl BackendRenderTarget {
    /// `image_info` is a raw `GrVkImageInfo`; construct it directly
    /// (`sys::GrVkImageInfo { fImage: ..., ..Default::default() }`).
    /// `sampleCnt` is taken from `image_info.fSampleCount` and `stencilBits`
    /// is always 0 -- Skia doesn't accept them as separate params here.
    pub fn new_vk(width: i32, height: i32, image_info: &sys::GrVkImageInfo) -> Self {
        let ptr = unsafe { sys::skialin_bridge_BackendRenderTarget_MakeVk(width, height, image_info) };
        BackendRenderTarget(ptr)
    }

    /// `gl_info` is a raw `GrGLFramebufferInfo`; construct it directly
    /// (`sys::GrGLFramebufferInfo { fFBOID: ..., ..Default::default() }`).
    pub fn new_gl(width: i32, height: i32, sample_cnt: i32, stencil_bits: i32, gl_info: &sys::GrGLFramebufferInfo) -> Self {
        let ptr = unsafe { sys::skialin_bridge_BackendRenderTarget_MakeGL(width, height, sample_cnt, stencil_bits, gl_info) };
        BackendRenderTarget(ptr)
    }

    pub fn width(&self) -> i32 {
        unsafe { sys::skialin_bridge_BackendRenderTarget_width(self.0) }
    }

    pub fn height(&self) -> i32 {
        unsafe { sys::skialin_bridge_BackendRenderTarget_height(self.0) }
    }

    pub fn sample_cnt(&self) -> i32 {
        unsafe { sys::skialin_bridge_BackendRenderTarget_sampleCnt(self.0) }
    }

    pub fn stencil_bits(&self) -> i32 {
        unsafe { sys::skialin_bridge_BackendRenderTarget_stencilBits(self.0) }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { sys::skialin_bridge_BackendRenderTarget_isValid(self.0) }
    }

    pub fn is_protected(&self) -> bool {
        unsafe { sys::skialin_bridge_BackendRenderTarget_isProtected(self.0) }
    }

    pub fn is_framebuffer_only(&self) -> bool {
        unsafe { sys::skialin_bridge_BackendRenderTarget_isFramebufferOnly(self.0) }
    }
}

impl Clone for BackendRenderTarget {
    fn clone(&self) -> Self {
        BackendRenderTarget(unsafe { sys::skialin_bridge_BackendRenderTarget_clone(self.0) })
    }
}

impl Drop for BackendRenderTarget {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_BackendRenderTarget_delete(self.0) };
    }
}
