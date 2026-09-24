use crate::sys;
use crate::GraphiteBackendTexture;

/// Wraps a skgpu::graphite::Context (Vulkan, or D3D12 through Dawn on Windows). Thread-safe and
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

    /// Dawn owns the D3D12 device it creates -- unlike `new_vulkan`, there is no caller-supplied
    /// device to hand in, so this enumerates D3D12 adapters itself. `adapter_index` selects among
    /// them (0 for the default/first). `d3d12_device`/`d3d12_command_queue` on the result are the
    /// raw COM objects Dawn is driving, valid for as long as the returned context is alive.
    /// Always `None` off Windows, where Dawn isn't built.
    pub fn new_dawn_d3d12(adapter_index: u32) -> Option<DawnD3D12Context> {
        let mut keep_alive_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        let mut d3d12_device: *mut std::ffi::c_void = std::ptr::null_mut();
        let mut d3d12_command_queue: *mut std::ffi::c_void = std::ptr::null_mut();
        let ptr = unsafe {
            sys::skialin_bridge_GraphiteContext_MakeDawnD3D12(adapter_index, &mut keep_alive_ptr, &mut d3d12_device, &mut d3d12_command_queue)
        };
        if ptr.is_null() {
            return None;
        }
        let keep_alive: Box<dyn std::any::Any> = Box::new(DawnKeepAlive(keep_alive_ptr));
        Some(DawnD3D12Context {
            context: GraphiteContext(ptr, Some(keep_alive)),
            d3d12_device: d3d12_device as i64,
            d3d12_command_queue: d3d12_command_queue as i64,
        })
    }

    /// Wraps a caller-owned `ID3D12Resource` (created on `DawnD3D12Context::d3d12_device`) as a
    /// `GraphiteBackendTexture`, via this context's Dawn device. `None` if this context wasn't
    /// made with `new_dawn_d3d12`, or the import fails. `dawn_texture_format`/`dawn_texture_usage`
    /// are `wgpu::TextureFormat`/`wgpu::TextureUsage` values, not `DXGI_FORMAT`.
    #[allow(clippy::too_many_arguments)]
    pub fn make_d3d12_backend_texture(
        &self,
        d3d12_resource: i64,
        width: i32,
        height: i32,
        sample_count: i32,
        mipmapped: bool,
        dawn_texture_format: u32,
        dawn_texture_usage: u32,
    ) -> Option<GraphiteBackendTexture> {
        let keep_alive = self.1.as_ref()?.downcast_ref::<DawnKeepAlive>()?;
        let mut texture_keep_alive: *mut std::ffi::c_void = std::ptr::null_mut();
        let ptr = unsafe {
            sys::skialin_bridge_GraphiteBackendTexture_MakeD3D12Resource(
                keep_alive.0,
                d3d12_resource as *mut std::ffi::c_void,
                width,
                height,
                sample_count,
                mipmapped,
                dawn_texture_format,
                dawn_texture_usage,
                &mut texture_keep_alive,
            )
        };
        if ptr.is_null() {
            return None;
        }
        Some(GraphiteBackendTexture(ptr, Some(Box::new(DawnTextureKeepAlive(texture_keep_alive)))))
    }
}

/// Keeps a Dawn-backed `GraphiteContext`'s `wgpu::Instance`/`wgpu::Device` alive for as long as
/// the context is; see `GraphiteContext::new_dawn_d3d12`. Stashed in `GraphiteContext`'s own
/// keep-alive slot, so it never needs to be named outside this file.
struct DawnKeepAlive(*mut std::ffi::c_void);

impl Drop for DawnKeepAlive {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_DawnKeepAlive_delete(self.0) };
    }
}

/// Holds the `wgpu::Texture` a Dawn-imported `GraphiteBackendTexture` wraps (Skia's
/// BackendTexture doesn't retain it); see `GraphiteContext::make_d3d12_backend_texture`.
struct DawnTextureKeepAlive(*mut std::ffi::c_void);

impl Drop for DawnTextureKeepAlive {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_DawnTextureKeepAlive_delete(self.0) };
    }
}

pub struct DawnD3D12Context {
    pub context: GraphiteContext,
    pub d3d12_device: i64,
    pub d3d12_command_queue: i64,
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
