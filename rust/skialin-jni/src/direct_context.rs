use jni::sys::{jboolean, jint, jlong};
use jni::JNIEnv;

use skialin_core::DirectContext;

use crate::util::{borrow_mut, box_ptr, drop_ptr};
use crate::vulkan_loader;

#[no_mangle]
pub extern "system" fn Java_org_skialin_DirectContextNative_nMakeGL(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    match DirectContext::new_gl() {
        Some(context) => box_ptr(context),
        None => 0,
    }
}

#[no_mangle]
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
) -> jlong {
    let Some(proc_ctx) = vulkan_loader::ProcAddrCtx::new(instance as _) else {
        return 0;
    };
    let context = DirectContext::new_vulkan(
        instance as _,
        physical_device as _,
        device as _,
        queue as _,
        graphics_queue_index as u32,
        max_api_version as u32,
        Box::new(proc_ctx),
        Some(vulkan_loader::get_proc),
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
