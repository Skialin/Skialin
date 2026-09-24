use jni::objects::JObject;
use jni::sys::{jboolean, jint, jlong};
use jni::JNIEnv;

use skialin_core::{GraphiteContext, GraphiteRecorder, GraphiteRecording, Surface};

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};
use crate::vulkan_loader;

#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub extern "system" fn Java_org_skialin_GraphiteContextNative_nMakeVulkan(
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
pub extern "system" fn Java_org_skialin_GraphiteContextNative_nMakeVulkanWithGetProc(
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
    let context = GraphiteContext::new_vulkan(
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
pub extern "system" fn Java_org_skialin_GraphiteContextNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<GraphiteContext>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_GraphiteContextNative_nMakeRecorder(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    match unsafe { borrow_mut::<GraphiteContext>(ptr) }.make_recorder() {
        Some(recorder) => box_ptr(recorder),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_GraphiteContextNative_nInsertRecording(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    recording_ptr: jlong,
    target_surface_ptr: jlong,
) -> jint {
    let context = unsafe { borrow_mut::<GraphiteContext>(ptr) };
    let recording = unsafe { borrow_mut::<GraphiteRecording>(recording_ptr) };
    let surface = unsafe { borrow_mut::<Surface>(target_surface_ptr) };
    context.insert_recording(recording, surface)
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_GraphiteContextNative_nSubmit(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, sync_to_cpu: jboolean) -> jboolean {
    unsafe { borrow_mut::<GraphiteContext>(ptr) }.submit(sync_to_cpu != 0) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_GraphiteContextNative_nMakeDawnD3D12<'l>(env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, adapter_index: jint) -> jni::sys::jlongArray {
    let array = env.new_long_array(3).expect("new_long_array");
    if let Some(dawn) = GraphiteContext::new_dawn_d3d12(adapter_index as u32) {
        env.set_long_array_region(&array, 0, &[box_ptr(dawn.context), dawn.d3d12_device, dawn.d3d12_command_queue]).expect("set_long_array_region");
    } else {
        env.set_long_array_region(&array, 0, &[0i64, 0, 0]).expect("set_long_array_region");
    }
    array.into_raw()
}

#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub extern "system" fn Java_org_skialin_GraphiteContextNative_nMakeD3D12BackendTexture(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    d3d12_resource: jlong,
    width: jint,
    height: jint,
    sample_count: jint,
    mipmapped: jboolean,
    dawn_texture_format: jint,
    dawn_texture_usage: jint,
) -> jlong {
    let context = unsafe { borrow::<GraphiteContext>(ptr) };
    match context.make_d3d12_backend_texture(d3d12_resource, width, height, sample_count, mipmapped != 0, dawn_texture_format as u32, dawn_texture_usage as u32) {
        Some(texture) => box_ptr(texture),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_GraphiteRecorderNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<GraphiteRecorder>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_GraphiteRecorderNative_nSnap(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    match unsafe { borrow_mut::<GraphiteRecorder>(ptr) }.snap() {
        Some(recording) => box_ptr(recording),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_GraphiteRecordingNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<GraphiteRecording>(ptr) };
}
