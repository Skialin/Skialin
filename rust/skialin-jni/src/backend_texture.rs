use jni::objects::JString;
use jni::sys::{jboolean, jint, jlong};
use jni::JNIEnv;

use skialin_core::sys;
use skialin_core::BackendTexture;

use crate::util::{borrow, box_ptr, drop_ptr};

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
