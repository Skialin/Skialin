use jni::objects::JString;
use jni::sys::{jboolean, jint, jlong};
use jni::JNIEnv;

use skialin_core::sys;
use skialin_core::BackendTexture;

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};

#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub extern "system" fn Java_org_skialin_BackendTextureNative_nMakeVk(
    mut env: JNIEnv,
    _class: jni::objects::JClass,
    width: jint,
    height: jint,
    image: jlong,
    image_tiling: jint,
    image_layout: jint,
    format: jint,
    image_usage_flags: jint,
    sample_count: jint,
    level_count: jint,
    current_queue_family: jint,
    is_protected: jboolean,
    sharing_mode: jint,
    label: JString,
) -> jlong {
    let label: String = env.get_string(&label).map(|s| s.into()).unwrap_or_default();
    let image_info = sys::GrVkImageInfo {
        fImage: image as sys::VkImage,
        fImageTiling: image_tiling as sys::VkImageTiling,
        fImageLayout: image_layout as sys::VkImageLayout,
        fFormat: format as sys::VkFormat,
        fImageUsageFlags: image_usage_flags as sys::VkImageUsageFlags,
        fSampleCount: sample_count as u32,
        fLevelCount: level_count as u32,
        fCurrentQueueFamily: current_queue_family as u32,
        fProtected: is_protected != 0,
        fSharingMode: sharing_mode as sys::VkSharingMode,
        ..Default::default()
    };
    box_ptr(BackendTexture::new_vk(width, height, &image_info, &label))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BackendTextureNative_nMakeGL(
    mut env: JNIEnv,
    _class: jni::objects::JClass,
    width: jint,
    height: jint,
    mipmapped: jboolean,
    target: jint,
    id: jint,
    format: jint,
    is_protected: jboolean,
    label: JString,
) -> jlong {
    let label: String = env.get_string(&label).map(|s| s.into()).unwrap_or_default();
    let gl_info =
        sys::GrGLTextureInfo { fTarget: target as sys::GrGLenum, fID: id as sys::GrGLuint, fFormat: format as sys::GrGLenum, fProtected: is_protected != 0 };
    box_ptr(BackendTexture::new_gl(width, height, mipmapped != 0, &gl_info, &label))
}

#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub extern "system" fn Java_org_skialin_BackendTextureNative_nMakeD3D(
    mut env: JNIEnv,
    _class: jni::objects::JClass,
    width: jint,
    height: jint,
    resource: jlong,
    resource_state: jint,
    format: jint,
    sample_count: jint,
    level_count: jint,
    sample_quality_pattern: jint,
    is_protected: jboolean,
    label: JString,
) -> jlong {
    let label: String = env.get_string(&label).map(|s| s.into()).unwrap_or_default();
    let texture = BackendTexture::new_d3d(
        width,
        height,
        resource as _,
        resource_state as u32,
        format as u32,
        sample_count as u32,
        level_count as u32,
        sample_quality_pattern as u32,
        is_protected != 0,
        &label,
    );
    match texture {
        Some(texture) => box_ptr(texture),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BackendTextureNative_nSetD3DResourceState(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, resource_state: jint) {
    unsafe { borrow_mut::<BackendTexture>(ptr) }.set_d3d_resource_state(resource_state as u32);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BackendTextureNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<BackendTexture>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BackendTextureNative_nWidth(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<BackendTexture>(ptr) }.width()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BackendTextureNative_nHeight(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<BackendTexture>(ptr) }.height()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BackendTextureNative_nIsValid(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<BackendTexture>(ptr) }.is_valid() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BackendTextureNative_nIsProtected(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<BackendTexture>(ptr) }.is_protected() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BackendTextureNative_nHasMipmaps(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<BackendTexture>(ptr) }.has_mipmaps() as jboolean
}
