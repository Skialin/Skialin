use crate::{sys, BlendMode};

pub struct Blender(pub(crate) *mut sys::SkBlender);

impl Blender {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::SkBlender) -> Option<Self> {
        (!ptr.is_null()).then_some(Blender(ptr))
    }

    pub fn mode(mode: BlendMode) -> Self {
        unsafe { Self::from_raw(sys::skialin_bridge_Blender_Mode(mode.into())) }.expect("Mode never returns null")
    }
}

impl Drop for Blender {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Blender_unref(self.0) };
    }
}
