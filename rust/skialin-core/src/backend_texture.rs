use crate::sys;

/// Wraps a GrBackendTexture: a handle to an existing GPU texture (as
/// opposed to one Skia allocated itself), for rendering into a
/// caller-supplied texture via `Surface::wrap_backend_texture`.
pub struct BackendTexture(pub(crate) *mut sys::GrBackendTexture);

impl BackendTexture {
    /// `image_info` is a raw `GrVkImageInfo`; construct it directly
    /// (`sys::GrVkImageInfo { fImage: ..., ..Default::default() }`).
    pub fn new_vk(width: i32, height: i32, image_info: &sys::GrVkImageInfo, label: &str) -> Self {
        let ptr = unsafe { sys::skialin_bridge_BackendTexture_MakeVk(width, height, image_info, label.as_ptr() as *const std::ffi::c_char, label.len()) };
        BackendTexture(ptr)
    }

    pub fn width(&self) -> i32 {
        unsafe { sys::skialin_bridge_BackendTexture_width(self.0) }
    }

    pub fn height(&self) -> i32 {
        unsafe { sys::skialin_bridge_BackendTexture_height(self.0) }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { sys::skialin_bridge_BackendTexture_isValid(self.0) }
    }

    pub fn is_protected(&self) -> bool {
        unsafe { sys::skialin_bridge_BackendTexture_isProtected(self.0) }
    }

    pub fn has_mipmaps(&self) -> bool {
        unsafe { sys::skialin_bridge_BackendTexture_hasMipmaps(self.0) }
    }
}

impl Clone for BackendTexture {
    fn clone(&self) -> Self {
        BackendTexture(unsafe { sys::skialin_bridge_BackendTexture_clone(self.0) })
    }
}

impl Drop for BackendTexture {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_BackendTexture_delete(self.0) };
    }
}
