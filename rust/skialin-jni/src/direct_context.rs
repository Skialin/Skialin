use jni::objects::JObject;
use jni::sys::{jboolean, jint, jlong};
use jni::JNIEnv;

use skialin_core::DirectContext;

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};
use crate::vulkan_loader;

#[no_mangle]
pub extern "system" fn Java_org_skialin_DirectContextNative_nMakeGL(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    match DirectContext::new_gl() {
        Some(context) => box_ptr(context),
        None => 0,
    }
}

#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub extern "system" fn Java_org_skialin_DirectContextNative_nMakeVulkan(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    instance: jlong,
    physical_device: jlong,
    device: jlong,
    queue: jlong,
    graphics_queue_index: jint,
    max_api_version: jint,
    protected_context: jboolean,
    get_instance_proc_addr: jlong,
    get_device_proc_addr: jlong,
) -> jlong {
    let proc_ctx = unsafe { vulkan_loader::ProcAddrCtx::from_pointers(get_instance_proc_addr as usize, get_device_proc_addr as usize, instance as _) };
    let Some(proc_ctx) = proc_ctx else {
        return 0;
    };
    make_vulkan(
        instance,
        physical_device,
        device,
        queue,
        graphics_queue_index,
        max_api_version,
        protected_context,
        Box::new(proc_ctx),
        Some(vulkan_loader::get_proc),
    )
}

#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub extern "system" fn Java_org_skialin_DirectContextNative_nMakeVulkanWithGetProc(
    env: JNIEnv,
    _class: jni::objects::JClass,
    instance: jlong,
    physical_device: jlong,
    device: jlong,
    queue: jlong,
    graphics_queue_index: jint,
    max_api_version: jint,
    protected_context: jboolean,
    get_proc: JObject,
) -> jlong {
    let Some(proc_ctx) = vulkan_loader::JvmProcAddrCtx::new(&env, get_proc) else {
        return 0;
    };
    make_vulkan(
        instance,
        physical_device,
        device,
        queue,
        graphics_queue_index,
        max_api_version,
        protected_context,
        Box::new(proc_ctx),
        Some(vulkan_loader::get_proc_jvm),
    )
}

#[allow(clippy::too_many_arguments)]
fn make_vulkan(
    instance: jlong,
    physical_device: jlong,
    device: jlong,
    queue: jlong,
    graphics_queue_index: jint,
    max_api_version: jint,
    protected_context: jboolean,
    proc_ctx: Box<dyn std::any::Any>,
    get_proc: skialin_core::sys::SkialinVulkanGetProc,
) -> jlong {
    let context = DirectContext::new_vulkan(
        instance as _,
        physical_device as _,
        device as _,
        queue as _,
        graphics_queue_index as u32,
        max_api_version as u32,
        proc_ctx,
        get_proc,
        protected_context != 0,
    );
    match context {
        Some(context) => box_ptr(context),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DirectContextNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<DirectContext>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DirectContextNative_nFlush(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { borrow_mut::<DirectContext>(ptr) }.flush();
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DirectContextNative_nSubmit(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, sync_cpu: jboolean) {
    unsafe { borrow_mut::<DirectContext>(ptr) }.submit(sync_cpu != 0);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DirectContextNative_nAbandonContext(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { borrow_mut::<DirectContext>(ptr) }.abandon_context();
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DirectContextNative_nResetAll(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { borrow_mut::<DirectContext>(ptr) }.reset_all();
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DirectContextNative_nGetResourceCacheLimit(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    unsafe { borrow::<DirectContext>(ptr) }.resource_cache_limit()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DirectContextNative_nSetResourceCacheLimit(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, max_resource_bytes: jlong) {
    unsafe { borrow_mut::<DirectContext>(ptr) }.set_resource_cache_limit(max_resource_bytes);
}
