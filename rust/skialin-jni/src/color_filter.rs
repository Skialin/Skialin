use jni::sys::{jfloat, jfloatArray, jint, jlong};
use jni::JNIEnv;

use skialin_core::ColorFilter;

use crate::paint::blend_mode_from_ordinal;
use crate::util::{borrow, box_ptr, drop_ptr};

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorFilterNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<ColorFilter>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorFilterNative_nBlend(_env: JNIEnv, _class: jni::objects::JClass, color: jint, mode: jint) -> jlong {
    match ColorFilter::blend(color as u32, blend_mode_from_ordinal(mode)) {
        Some(filter) => box_ptr(filter),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorFilterNative_nMatrix(env: JNIEnv, _class: jni::objects::JClass, row_major_20: jfloatArray, clamp: jni::sys::jboolean) -> jlong {
    let array = unsafe { jni::objects::JFloatArray::from_raw(row_major_20) };
    let mut values = [0f32; 20];
    env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
    match ColorFilter::matrix(&values, clamp != 0) {
        Some(filter) => box_ptr(filter),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorFilterNative_nCompose(_env: JNIEnv, _class: jni::objects::JClass, outer_ptr: jlong, inner_ptr: jlong) -> jlong {
    let outer = unsafe { borrow::<ColorFilter>(outer_ptr) };
    let inner = unsafe { borrow::<ColorFilter>(inner_ptr) };
    match ColorFilter::compose(outer, inner) {
        Some(filter) => box_ptr(filter),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorFilterNative_nLerp(_env: JNIEnv, _class: jni::objects::JClass, t: jfloat, dst_ptr: jlong, src_ptr: jlong) -> jlong {
    let dst = unsafe { borrow::<ColorFilter>(dst_ptr) };
    let src = unsafe { borrow::<ColorFilter>(src_ptr) };
    match ColorFilter::lerp(t, dst, src) {
        Some(filter) => box_ptr(filter),
        None => 0,
    }
}
