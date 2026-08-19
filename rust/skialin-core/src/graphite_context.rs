use crate::sys;

/// Wraps a skgpu::graphite::Context (Vulkan only). Thread-safe and
/// long-lived, unlike a Recorder made from it. The caller creates the
/// Vulkan instance/device/queue and supplies a proc-address resolver, same
/// as `DirectContext::new_vulkan`.
pub struct GraphiteContext(pub(crate) *mut sys::skgpu::graphite::Context, #[allow(dead_code)] Option<Box<dyn std::any::Any>>);

impl GraphiteContext {
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
            sys::skialin_bridge_GraphiteContext_MakeVulkan(
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
        (!ptr.is_null()).then_some(GraphiteContext(ptr, Some(keep_alive)))
    }

    pub fn make_recorder(&mut self) -> Option<GraphiteRecorder> {
        let ptr = unsafe { sys::skialin_bridge_GraphiteContext_makeRecorder(self.0) };
        (!ptr.is_null()).then_some(GraphiteRecorder(ptr))
    }

    /// Returns the real `skgpu::graphite::InsertStatus::V` value (0 == success).
    pub fn insert_recording(&mut self, recording: &mut GraphiteRecording, target_surface: &mut crate::Surface) -> i32 {
        unsafe { sys::skialin_bridge_GraphiteContext_insertRecording(self.0, recording.0, target_surface.0) }
    }

    pub fn submit(&mut self, sync_to_cpu: bool) -> bool {
        unsafe { sys::skialin_bridge_GraphiteContext_submit(self.0, sync_to_cpu) }
    }
}

impl Drop for GraphiteContext {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_GraphiteContext_delete(self.0) };
    }
}

/// Not thread-safe; one per thread/frame. Records draws into a
/// `GraphiteRecording` via `snap`, which is then inserted back into the
/// `GraphiteContext` that created this Recorder.
pub struct GraphiteRecorder(pub(crate) *mut sys::skgpu::graphite::Recorder);

impl GraphiteRecorder {
    pub fn snap(&mut self) -> Option<GraphiteRecording> {
        let ptr = unsafe { sys::skialin_bridge_GraphiteRecorder_snap(self.0) };
        (!ptr.is_null()).then_some(GraphiteRecording(ptr))
    }
}

impl Drop for GraphiteRecorder {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_GraphiteRecorder_delete(self.0) };
    }
}

pub struct GraphiteRecording(pub(crate) *mut sys::skgpu::graphite::Recording);

impl Drop for GraphiteRecording {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_GraphiteRecording_delete(self.0) };
    }
}
