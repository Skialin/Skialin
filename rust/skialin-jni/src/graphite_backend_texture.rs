use jni::sys::{jboolean, jint, jlong};
use jni::JNIEnv;

use skialin_core::GraphiteBackendTexture;

use crate::util::{borrow, box_ptr, drop_ptr};

#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub extern "system" fn Java_org_skialin_GraphiteBackendTextureNative_nMakeVk(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    width: jint,
    height: jint,
    sample_count: jint,
    mipmapped: jboolean,
    image_create_flags: jint,
    format: jint,
    image_tiling: jint,
    image_usage_flags: jint,
    sharing_mode: jint,
    aspect_mask: jint,
    current_layout: jint,
    queue_family_index: jint,
    image: jlong,
    alloc_memory: jlong,
    alloc_offset: jlong,
    alloc_size: jlong,
    alloc_flags: jint,
) -> jlong {
    let texture = GraphiteBackendTexture::new_vk(
        width,
        height,
        sample_count,
        mipmapped != 0,
        image_create_flags as u32,
        format as _,
        image_tiling as _,
        image_usage_flags as _,
        sharing_mode as _,
        aspect_mask as _,
        current_layout as _,
        queue_family_index as u32,
        image as _,
        alloc_memory as _,
        alloc_offset as _,
        alloc_size as _,
        alloc_flags as u32,
    );
    box_ptr(texture)
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_GraphiteBackendTextureNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<GraphiteBackendTexture>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_GraphiteBackendTextureNative_nIsValid(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<GraphiteBackendTexture>(ptr) }.is_valid() as jboolean
}
