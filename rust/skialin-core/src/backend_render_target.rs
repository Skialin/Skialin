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

    /// Same as `BackendTexture::new_d3d`, e.g. for a swapchain buffer. Always `None` off Windows.
    #[allow(clippy::too_many_arguments)]
    pub fn new_d3d(
        width: i32,
        height: i32,
        resource: *mut std::ffi::c_void,
        resource_state: u32,
        format: u32,
        sample_count: u32,
        level_count: u32,
        sample_quality_pattern: u32,
        is_protected: bool,
    ) -> Option<Self> {
        let ptr = unsafe {
            sys::skialin_bridge_BackendRenderTarget_MakeD3D(
                width,
                height,
                resource,
                resource_state,
                format,
                sample_count,
                level_count,
                sample_quality_pattern,
                is_protected,
            )
        };
        (!ptr.is_null()).then_some(BackendRenderTarget(ptr))
    }

    /// Wraps a caller-owned `id<MTLTexture>` render target (e.g. a `CAMetalDrawable`'s texture),
    /// retained for as long as Skia needs it. Always `None` off macOS.
    pub fn new_metal(width: i32, height: i32, texture: *mut std::ffi::c_void) -> Option<Self> {
        let ptr = unsafe { sys::skialin_bridge_BackendRenderTarget_MakeMtl(width, height, texture) };
        (!ptr.is_null()).then_some(BackendRenderTarget(ptr))
    }

    /// Tells Skia the caller transitioned the wrapped `ID3D12Resource` to `resource_state`. No-op
    /// for non-D3D render targets.
    pub fn set_d3d_resource_state(&mut self, resource_state: u32) {
        unsafe { sys::skialin_bridge_BackendRenderTarget_setD3DResourceState(self.0, resource_state) };
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
