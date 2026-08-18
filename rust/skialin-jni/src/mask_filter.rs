use jni::sys::{jboolean, jfloat, jint, jlong};
use jni::JNIEnv;

use skialin_core::{BlurStyle, MaskFilter};

use crate::util::drop_ptr;

fn blur_style_from_ordinal(ordinal: jint) -> BlurStyle {
    match ordinal {
        1 => BlurStyle::Solid,
        2 => BlurStyle::Outer,
        3 => BlurStyle::Inner,
        _ => BlurStyle::Normal,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_MaskFilterNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<MaskFilter>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_MaskFilterNative_nBlur(_env: JNIEnv, _class: jni::objects::JClass, style: jint, sigma: jfloat, respect_ctm: jboolean) -> jlong {
    match MaskFilter::blur(blur_style_from_ordinal(style), sigma, respect_ctm != 0) {
        Some(filter) => crate::util::box_ptr(filter),
        None => 0,
    }
}
