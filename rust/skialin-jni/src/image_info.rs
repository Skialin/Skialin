use jni::sys::{jboolean, jint, jlong};
use jni::JNIEnv;

use skialin_core::ImageInfo;

use crate::color_space::color_space_ptr_from_jlong;
use crate::color_type::{alpha_type_from_ordinal, alpha_type_to_ordinal, color_type_from_ordinal, color_type_to_ordinal};
use crate::util::{borrow, box_ptr, drop_ptr};

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageInfoNative_nMake(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    width: jint,
    height: jint,
    color_type: jint,
    alpha_type: jint,
    color_space_ptr: jlong,
) -> jlong {
    let cs = color_space_ptr_from_jlong(color_space_ptr);
    box_ptr(ImageInfo::with_color_space(width, height, color_type_from_ordinal(color_type), alpha_type_from_ordinal(alpha_type), cs))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageInfoNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<ImageInfo>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageInfoNative_nWidth(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<ImageInfo>(ptr) }.width()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageInfoNative_nHeight(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<ImageInfo>(ptr) }.height()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageInfoNative_nColorType(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    color_type_to_ordinal(unsafe { borrow::<ImageInfo>(ptr) }.color_type())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageInfoNative_nAlphaType(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    alpha_type_to_ordinal(unsafe { borrow::<ImageInfo>(ptr) }.alpha_type())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageInfoNative_nIsEmpty(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<ImageInfo>(ptr) }.is_empty() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageInfoNative_nIsOpaque(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<ImageInfo>(ptr) }.is_opaque() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageInfoNative_nGammaCloseToSrgb(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<ImageInfo>(ptr) }.gamma_close_to_srgb() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageInfoNative_nBytesPerPixel(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<ImageInfo>(ptr) }.bytes_per_pixel()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageInfoNative_nShiftPerPixel(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<ImageInfo>(ptr) }.shift_per_pixel()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageInfoNative_nMinRowBytes(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    unsafe { borrow::<ImageInfo>(ptr) }.min_row_bytes() as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageInfoNative_nComputeMinByteSize(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    unsafe { borrow::<ImageInfo>(ptr) }.compute_min_byte_size() as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageInfoNative_nComputeByteSize(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, row_bytes: jlong) -> jlong {
    unsafe { borrow::<ImageInfo>(ptr) }.compute_byte_size(row_bytes as usize) as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageInfoNative_nValidRowBytes(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, row_bytes: jlong) -> jboolean {
    unsafe { borrow::<ImageInfo>(ptr) }.valid_row_bytes(row_bytes as usize) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageInfoNative_nWithWH(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, width: jint, height: jint) -> jlong {
    box_ptr(unsafe { borrow::<ImageInfo>(ptr) }.with_wh(width, height))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageInfoNative_nWithColorType(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, color_type: jint) -> jlong {
    box_ptr(unsafe { borrow::<ImageInfo>(ptr) }.with_color_type(color_type_from_ordinal(color_type)))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageInfoNative_nWithAlphaType(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, alpha_type: jint) -> jlong {
    box_ptr(unsafe { borrow::<ImageInfo>(ptr) }.with_alpha_type(alpha_type_from_ordinal(alpha_type)))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageInfoNative_nWithColorSpace(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, color_space_ptr: jlong) -> jlong {
    let cs = color_space_ptr_from_jlong(color_space_ptr);
    box_ptr(unsafe { borrow::<ImageInfo>(ptr) }.with_color_space_opt(cs))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageInfoNative_nEquals(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, other_ptr: jlong) -> jboolean {
    let a = unsafe { borrow::<ImageInfo>(ptr) };
    let b = unsafe { borrow::<ImageInfo>(other_ptr) };
    a.equals(b) as jboolean
}
