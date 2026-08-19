use skialin_core::sys;
use std::ffi::{c_char, c_void};
use std::sync::OnceLock;

type PfnGetInstanceProcAddr = unsafe extern "system" fn(*mut c_void, *const c_char) -> sys::PFN_vkVoidFunction;
type PfnGetDeviceProcAddr = unsafe extern "system" fn(*mut c_void, *const c_char) -> sys::PFN_vkVoidFunction;

#[cfg(target_os = "windows")]
const LIB_NAMES: &[&str] = &["vulkan-1.dll"];
#[cfg(target_os = "macos")]
const LIB_NAMES: &[&str] = &["libvulkan.dylib", "libvulkan.1.dylib", "libMoltenVK.dylib"];
#[cfg(all(unix, not(target_os = "macos")))]
const LIB_NAMES: &[&str] = &["libvulkan.so.1", "libvulkan.so"];

struct Loader {
    _lib: libloading::Library,
    get_instance_proc_addr: PfnGetInstanceProcAddr,
}

// SAFETY: the loaded function pointer is process-global and immutable once resolved.
unsafe impl Sync for Loader {}

fn loader() -> Option<&'static Loader> {
    static LOADER: OnceLock<Option<Loader>> = OnceLock::new();
    LOADER
        .get_or_init(|| {
            LIB_NAMES.iter().find_map(|name| unsafe {
                let lib = libloading::Library::new(name).ok()?;
                let get_instance_proc_addr = *lib.get::<PfnGetInstanceProcAddr>(b"vkGetInstanceProcAddr\0").ok()?;
                Some(Loader { _lib: lib, get_instance_proc_addr })
            })
        })
        .as_ref()
}

/// Per-DirectContext state for [get_proc]: vkGetDeviceProcAddr, resolved
/// once via the real instance at construction time. Skia calls fGetProc
/// with a null instance once it has a device (device-level lookups should
/// go through vkGetDeviceProcAddr, not be re-derived from the instance
/// argument, which won't be there anymore), so this can't be resolved lazily
/// per call the way the instance-level lookup can.
pub struct ProcAddrCtx {
    get_device_proc_addr: PfnGetDeviceProcAddr,
}

impl ProcAddrCtx {
    /// `instance` must be a valid, live VkInstance.
    pub fn new(instance: sys::VkInstance) -> Option<Self> {
        let loader = loader()?;
        let name = c"vkGetDeviceProcAddr";
        let get_device_proc_addr = unsafe { (loader.get_instance_proc_addr)(instance as *mut c_void, name.as_ptr()) }?;
        Some(ProcAddrCtx { get_device_proc_addr: unsafe { std::mem::transmute(get_device_proc_addr) } })
    }
}

/// Matches `SkialinVulkanGetProc`. `ctx` must point to a live `ProcAddrCtx`.
pub unsafe extern "C" fn get_proc(ctx: *mut c_void, name: *const c_char, instance: sys::VkInstance, device: sys::VkDevice) -> sys::PFN_vkVoidFunction {
    let Some(loader) = loader() else { return None };
    if !device.is_null() {
        let proc_ctx = &*(ctx as *const ProcAddrCtx);
        (proc_ctx.get_device_proc_addr)(device as *mut c_void, name)
    } else {
        (loader.get_instance_proc_addr)(instance as *mut c_void, name)
    }
}
