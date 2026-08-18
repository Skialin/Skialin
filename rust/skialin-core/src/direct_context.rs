use crate::sys;

/// Matches GrSurfaceOrigin's declaration order (include/gpu/ganesh/GrTypes.h).
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

/// Wraps a `GrDirectContext` (Ganesh + OpenGL). The native GL context it
/// wraps must already be current on the calling OS thread before
/// `new_gl` is called (e.g. via LWJGL/GLFW on the JVM side, or a
/// platform GL context in a Rust test) -- this type does not create one
/// itself. GrDirectContext is thread-affine after creation: every method
/// here, and every `Surface` created from it, must run on that same
/// thread for as long as the underlying GL context stays current there.
pub struct DirectContext(pub(crate) *mut sys::GrDirectContext);

impl DirectContext {
    pub fn new_gl() -> Option<Self> {
        let ptr = unsafe { sys::skialin_bridge_DirectContext_MakeGL() };
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
