use jni::sys::{jboolean, jint, jlong};
use jni::JNIEnv;

use skialin_core::sys;
use skialin_core::BackendRenderTarget;

use crate::util::{borrow, box_ptr, drop_ptr};

#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub extern "system" fn Java_org_skialin_BackendRenderTargetNative_nMakeVk(
    _env: JNIEnv,
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
) -> jlong {
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
    box_ptr(BackendRenderTarget::new_vk(width, height, &image_info))
}

#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub extern "system" fn Java_org_skialin_BackendRenderTargetNative_nMakeGL(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    width: jint,
    height: jint,
    sample_count: jint,
    stencil_bits: jint,
    fbo_id: jint,
    format: jint,
    is_protected: jboolean,
) -> jlong {
    let gl_info =
        sys::GrGLFramebufferInfo { fFBOID: fbo_id as sys::GrGLuint, fFormat: format as sys::GrGLenum, fProtected: is_protected != 0 };
    box_ptr(BackendRenderTarget::new_gl(width, height, sample_count, stencil_bits, &gl_info))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BackendRenderTargetNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<BackendRenderTarget>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BackendRenderTargetNative_nWidth(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<BackendRenderTarget>(ptr) }.width()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BackendRenderTargetNative_nHeight(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<BackendRenderTarget>(ptr) }.height()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BackendRenderTargetNative_nSampleCnt(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<BackendRenderTarget>(ptr) }.sample_cnt()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BackendRenderTargetNative_nStencilBits(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<BackendRenderTarget>(ptr) }.stencil_bits()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BackendRenderTargetNative_nIsValid(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<BackendRenderTarget>(ptr) }.is_valid() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BackendRenderTargetNative_nIsProtected(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<BackendRenderTarget>(ptr) }.is_protected() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BackendRenderTargetNative_nIsFramebufferOnly(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
) -> jboolean {
    unsafe { borrow::<BackendRenderTarget>(ptr) }.is_framebuffer_only() as jboolean
}
