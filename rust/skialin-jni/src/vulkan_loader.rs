use jni::objects::{GlobalRef, JObject, JValue};
use jni::sys::jlong;
use jni::{JNIEnv, JavaVM};
use skialin_core::sys;
use std::ffi::{c_char, c_void, CStr};
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

/// Per-context state for [get_proc]: vkGetInstanceProcAddr plus
/// vkGetDeviceProcAddr, both resolved once at construction time. Skia calls
/// fGetProc with a null instance once it has a device (device-level lookups
/// should go through vkGetDeviceProcAddr, not be re-derived from the instance
/// argument, which won't be there anymore), so the device-level entry point
/// can't be resolved lazily per call the way the instance-level one could.
pub struct ProcAddrCtx {
    get_instance_proc_addr: PfnGetInstanceProcAddr,
    get_device_proc_addr: PfnGetDeviceProcAddr,
}

impl ProcAddrCtx {
    /// Takes caller-supplied `vkGetInstanceProcAddr` /
    /// `vkGetDeviceProcAddr` function-pointer addresses. Either may be 0,
    /// meaning "resolve it the default way": the instance-level one falls
    /// back to the platform loader, the device-level one to a lookup through
    /// whichever instance-level entry point ends up in use. `instance` must
    /// be a valid, live VkInstance.
    ///
    /// # Safety
    /// Non-zero addresses must be live function pointers with the
    /// `vkGetInstanceProcAddr` / `vkGetDeviceProcAddr` signatures, and must
    /// stay valid for as long as the context built from this is alive.
    pub unsafe fn from_pointers(get_instance_proc_addr: usize, get_device_proc_addr: usize, instance: sys::VkInstance) -> Option<Self> {
        let get_instance_proc_addr: PfnGetInstanceProcAddr = match get_instance_proc_addr {
            0 => loader()?.get_instance_proc_addr,
            addr => std::mem::transmute::<usize, PfnGetInstanceProcAddr>(addr),
        };
        let get_device_proc_addr: PfnGetDeviceProcAddr = match get_device_proc_addr {
            0 => {
                let name = c"vkGetDeviceProcAddr";
                let resolved = get_instance_proc_addr(instance as *mut c_void, name.as_ptr())?;
                std::mem::transmute::<unsafe extern "C" fn(), PfnGetDeviceProcAddr>(resolved)
            }
            addr => std::mem::transmute::<usize, PfnGetDeviceProcAddr>(addr),
        };
        Some(ProcAddrCtx { get_instance_proc_addr, get_device_proc_addr })
    }
}

/// Matches `SkialinVulkanGetProc`. `ctx` must point to a live `ProcAddrCtx`.
pub unsafe extern "C" fn get_proc(ctx: *mut c_void, name: *const c_char, instance: sys::VkInstance, device: sys::VkDevice) -> sys::PFN_vkVoidFunction {
    let proc_ctx = &*(ctx as *const ProcAddrCtx);
    if !device.is_null() {
        (proc_ctx.get_device_proc_addr)(device as *mut c_void, name)
    } else {
        (proc_ctx.get_instance_proc_addr)(instance as *mut c_void, name)
    }
}

/// Per-context state for [get_proc_jvm]: a durable reference to the Kotlin
/// `VulkanGetProc` the caller handed to `makeVulkan`. Skia may resolve
/// symbols long after construction and from threads Skia owns, hence the
/// global ref and the per-call attach.
pub struct JvmProcAddrCtx {
    jvm: JavaVM,
    callback: GlobalRef,
}

impl JvmProcAddrCtx {
    pub fn new(env: &JNIEnv, callback: JObject) -> Option<Self> {
        let jvm = env.get_java_vm().ok()?;
        let callback = env.new_global_ref(callback).ok()?;
        Some(JvmProcAddrCtx { jvm, callback })
    }
}

/// Matches `SkialinVulkanGetProc`. `ctx` must point to a live `JvmProcAddrCtx`.
pub unsafe extern "C" fn get_proc_jvm(ctx: *mut c_void, name: *const c_char, instance: sys::VkInstance, device: sys::VkDevice) -> sys::PFN_vkVoidFunction {
    let proc_ctx = &*(ctx as *const JvmProcAddrCtx);
    let name = CStr::from_ptr(name).to_str().ok()?;
    let mut env = proc_ctx.jvm.attach_current_thread().ok()?;
    let name = env.new_string(name).ok()?;
    let args = [JValue::Object(&name), JValue::Long(instance as usize as jlong), JValue::Long(device as usize as jlong)];
    let result = env.call_method(proc_ctx.callback.as_obj(), "getProc", "(Ljava/lang/String;JJ)J", &args);
    // A pending exception would otherwise surface at the next JNI boundary,
    // far from the Kotlin code that actually threw.
    if env.exception_check().unwrap_or(false) {
        let _ = env.exception_describe();
        let _ = env.exception_clear();
        return None;
    }
    let addr = result.ok()?.j().ok()?;
    (addr != 0).then(|| std::mem::transmute::<usize, unsafe extern "C" fn()>(addr as usize))
}
