use crate::sys;

/// Matches GrSurfaceOrigin's declaration order.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SurfaceOrigin {
    TopLeft,
    BottomLeft,
}

impl From<SurfaceOrigin> for sys::GrSurfaceOrigin {
    fn from(origin: SurfaceOrigin) -> Self {
        match origin {
            SurfaceOrigin::TopLeft => 0,
            SurfaceOrigin::BottomLeft => 1,
        }
    }
}

/// skgpu::Origin has the same declaration order as GrSurfaceOrigin but is a
/// distinct type (used by Graphite APIs).
impl From<SurfaceOrigin> for sys::skgpu::Origin {
    fn from(origin: SurfaceOrigin) -> Self {
        match origin {
            SurfaceOrigin::TopLeft => 0,
            SurfaceOrigin::BottomLeft => 1,
        }
    }
}

/// Wraps a GrDirectContext (Ganesh + OpenGL or Vulkan). The caller must make
/// a native GL context current on this thread first for `new_gl*`; this type
/// doesn't create one. Thread-affine after creation for GL: every method
/// here, and every `Surface` made from it, must stay on that thread.
pub struct DirectContext(pub(crate) *mut sys::GrDirectContext, #[allow(dead_code)] Option<Box<dyn std::any::Any>>);

impl DirectContext {
    /// Resolves GL function pointers via Skia's own per-platform dispatch.
    pub fn new_gl() -> Option<Self> {
        let ptr = unsafe { sys::skialin_bridge_DirectContext_MakeGL() };
        (!ptr.is_null()).then_some(DirectContext(ptr, None))
    }

    /// Same as `new_gl`, but with a caller-supplied function pointer
    /// resolver instead of Skia's default loader.
    pub fn new_gl_assembled(ctx: *mut std::ffi::c_void, get: sys::GrGLGetProc) -> Option<Self> {
        let ptr = unsafe { sys::skialin_bridge_DirectContext_MakeGLAssembled(ctx, get) };
        (!ptr.is_null()).then_some(DirectContext(ptr, None))
    }

    /// Wraps a caller-created Vulkan instance/physical device/device/queue.
    /// Those must outlive this DirectContext and everything made from it.
    /// `get_proc` resolves function pointers (vkGetInstanceProcAddr /
    /// vkGetDeviceProcAddr) and may be called for as long as the returned
    /// DirectContext is alive, not just during this call, so `get_proc_ctx`
    /// is an owned value this DirectContext keeps alive for that long
    /// (reclaimed and dropped if construction fails).
    #[allow(clippy::too_many_arguments)]
    pub fn new_vulkan(
        instance: sys::VkInstance,
        physical_device: sys::VkPhysicalDevice,
        device: sys::VkDevice,
        queue: sys::VkQueue,
        graphics_queue_index: u32,
        max_api_version: u32,
        get_proc_ctx: Box<dyn std::any::Any>,
        get_proc: sys::SkialinVulkanGetProc,
        protected_context: bool,
    ) -> Option<Self> {
        let ctx_ptr = Box::into_raw(get_proc_ctx);
        let ptr = unsafe {
            sys::skialin_bridge_DirectContext_MakeVulkan(
                instance,
                physical_device,
                device,
                queue,
                graphics_queue_index,
                max_api_version,
                ctx_ptr as *mut std::ffi::c_void,
                get_proc,
                protected_context,
            )
        };
        let keep_alive = unsafe { Box::from_raw(ctx_ptr) };
        (!ptr.is_null()).then_some(DirectContext(ptr, Some(keep_alive)))
    }

    pub fn flush(&mut self) {
        unsafe { sys::skialin_bridge_DirectContext_flush(self.0) };
    }

    pub fn submit(&mut self, sync_cpu: bool) {
        unsafe { sys::skialin_bridge_DirectContext_submit(self.0, sync_cpu) };
    }

    pub fn abandon_context(&mut self) {
        unsafe { sys::skialin_bridge_DirectContext_abandonContext(self.0) };
    }

    /// Invalidates Ganesh's cached GL state (texture bindings, blend state,
    /// etc.) so the next Skia draw doesn't assume state left over from a
    /// prior frame. Call before drawing whenever GL state may have been
    /// changed by code outside Skia's control. No-op for Vulkan.
    pub fn reset_all(&mut self) {
        unsafe { sys::skialin_bridge_DirectContext_resetAll(self.0) };
    }

    pub fn resource_cache_limit(&self) -> i64 {
        unsafe { sys::skialin_bridge_DirectContext_getResourceCacheLimit(self.0) }
    }

    pub fn set_resource_cache_limit(&mut self, max_resource_bytes: i64) {
        unsafe { sys::skialin_bridge_DirectContext_setResourceCacheLimit(self.0, max_resource_bytes) };
    }
}

impl Drop for DirectContext {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_DirectContext_unref(self.0) };
    }
}
