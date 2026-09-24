use crate::sys;

/// Wraps a skgpu::graphite::BackendTexture: a caller-owned GPU texture (VkImage, or an
/// ID3D12Resource imported through Dawn) for rendering into via
/// `Surface::wrap_graphite_backend_texture`. The second field keeps alive whatever the native
/// BackendTexture borrows without retaining (Dawn's imported texture); fields drop after `Drop::drop`
/// runs, so it outlives the BackendTexture itself.
pub struct GraphiteBackendTexture(pub(crate) *mut sys::skgpu::graphite::BackendTexture, #[allow(dead_code)] pub(crate) Option<Box<dyn std::any::Any>>);

impl GraphiteBackendTexture {
    #[allow(clippy::too_many_arguments)]
    pub fn new_vk(
        width: i32,
        height: i32,
        sample_count: i32,
        mipmapped: bool,
        image_create_flags: u32,
        format: sys::VkFormat,
        image_tiling: sys::VkImageTiling,
        image_usage_flags: sys::VkImageUsageFlags,
        sharing_mode: sys::VkSharingMode,
        aspect_mask: sys::VkImageAspectFlags,
        current_layout: sys::VkImageLayout,
        queue_family_index: u32,
        image: sys::VkImage,
        alloc_memory: sys::VkDeviceMemory,
        alloc_offset: sys::VkDeviceSize,
        alloc_size: sys::VkDeviceSize,
        alloc_flags: u32,
    ) -> Self {
        let ptr = unsafe {
            sys::skialin_bridge_GraphiteBackendTexture_MakeVk(
                width,
                height,
                sample_count,
                mipmapped,
                image_create_flags,
                format,
                image_tiling,
                image_usage_flags,
                sharing_mode,
                aspect_mask,
                current_layout,
                queue_family_index,
                image,
                alloc_memory,
                alloc_offset,
                alloc_size,
                alloc_flags,
            )
        };
        GraphiteBackendTexture(ptr, None)
    }

    pub fn is_valid(&self) -> bool {
        unsafe { sys::skialin_bridge_GraphiteBackendTexture_isValid(self.0) }
    }
}

impl Drop for GraphiteBackendTexture {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_GraphiteBackendTexture_delete(self.0) };
    }
}
