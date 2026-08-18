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

/// Wraps a GrDirectContext (Ganesh + OpenGL). The caller must make a native
/// GL context current on this thread first; this type doesn't create one.
/// Thread-affine after creation: every method here, and every `Surface`
/// made from it, must stay on that thread.
pub struct DirectContext(pub(crate) *mut sys::GrDirectContext);

impl DirectContext {
    /// Resolves GL function pointers via Skia's own per-platform dispatch.
    pub fn new_gl() -> Option<Self> {
        let ptr = unsafe { sys::skialin_bridge_DirectContext_MakeGL() };
        (!ptr.is_null()).then_some(DirectContext(ptr))
    }

    /// Same as `new_gl`, but with a caller-supplied function pointer
    /// resolver instead of Skia's default loader.
    pub fn new_gl_assembled(ctx: *mut std::ffi::c_void, get: sys::GrGLGetProc) -> Option<Self> {
        let ptr = unsafe { sys::skialin_bridge_DirectContext_MakeGLAssembled(ctx, get) };
        (!ptr.is_null()).then_some(DirectContext(ptr))
    }

    /// Wraps a caller-created Vulkan instance/physical device/device/queue.
    /// Those must outlive this DirectContext and everything made from it.
    /// `get_proc` resolves function pointers (vkGetInstanceProcAddr /
    /// vkGetDeviceProcAddr); `get_proc_ctx` is passed through uninterpreted.
    #[allow(clippy::too_many_arguments)]
    pub fn new_vulkan(
        instance: sys::VkInstance,
        physical_device: sys::VkPhysicalDevice,
        device: sys::VkDevice,
        queue: sys::VkQueue,
        graphics_queue_index: u32,
        max_api_version: u32,
        get_proc_ctx: *mut std::ffi::c_void,
        get_proc: sys::SkialinVulkanGetProc,
        protected_context: bool,
    ) -> Option<Self> {
        let ptr = unsafe {
            sys::skialin_bridge_DirectContext_MakeVulkan(
                instance,
                physical_device,
                device,
                queue,
                graphics_queue_index,
                max_api_version,
                get_proc_ctx,
                get_proc,
                protected_context,
            )
        };
        (!ptr.is_null()).then_some(DirectContext(ptr))
    }

    pub fn flush(&mut self) {
        unsafe { sys::skialin_bridge_DirectContext_flush(self.0) };
    }

    pub fn submit(&mut self, sync_cpu: bool) {
        unsafe { sys::skialin_bridge_DirectContext_submit(self.0, sync_cpu) };
    }
}

impl Drop for DirectContext {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_DirectContext_unref(self.0) };
    }
}
