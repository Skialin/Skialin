use jni::sys::{jfloat, jfloatArray, jint, jlong};
use jni::JNIEnv;

use skialin_core::{Matrix, Path, Path1DStyle, PathEffect, TrimMode};

use crate::util::{borrow, box_ptr, drop_ptr};

fn path_1d_style_from_ordinal(ordinal: jint) -> Path1DStyle {
    match ordinal {
        1 => Path1DStyle::Rotate,
        2 => Path1DStyle::Morph,
        _ => Path1DStyle::Translate,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathEffectNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<PathEffect>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathEffectNative_nDash(env: JNIEnv, _class: jni::objects::JClass, intervals: jfloatArray, phase: jfloat) -> jlong {
    let array = unsafe { jni::objects::JFloatArray::from_raw(intervals) };
    let len = env.get_array_length(&array).expect("get_array_length") as usize;
    let mut buf = vec![0f32; len];
    env.get_float_array_region(&array, 0, &mut buf).expect("get_float_array_region");
    match PathEffect::dash(&buf, phase) {
        Some(effect) => box_ptr(effect),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathEffectNative_nCorner(_env: JNIEnv, _class: jni::objects::JClass, radius: jfloat) -> jlong {
    match PathEffect::corner(radius) {
        Some(effect) => box_ptr(effect),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathEffectNative_nDiscrete(_env: JNIEnv, _class: jni::objects::JClass, seg_length: jfloat, deviation: jfloat, seed_assist: jint) -> jlong {
    match PathEffect::discrete(seg_length, deviation, seed_assist as u32) {
        Some(effect) => box_ptr(effect),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathEffectNative_nTrim(_env: JNIEnv, _class: jni::objects::JClass, start_t: jfloat, stop_t: jfloat, mode: jint) -> jlong {
    let mode = if mode == 1 { TrimMode::Inverted } else { TrimMode::Normal };
    match PathEffect::trim(start_t, stop_t, mode) {
        Some(effect) => box_ptr(effect),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathEffectNative_nCompose(_env: JNIEnv, _class: jni::objects::JClass, outer_ptr: jlong, inner_ptr: jlong) -> jlong {
    let outer = unsafe { borrow::<PathEffect>(outer_ptr) };
    let inner = unsafe { borrow::<PathEffect>(inner_ptr) };
    match PathEffect::compose(outer, inner) {
        Some(effect) => box_ptr(effect),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathEffectNative_nSum(_env: JNIEnv, _class: jni::objects::JClass, first_ptr: jlong, second_ptr: jlong) -> jlong {
    let first = unsafe { borrow::<PathEffect>(first_ptr) };
    let second = unsafe { borrow::<PathEffect>(second_ptr) };
    match PathEffect::sum(first, second) {
        Some(effect) => box_ptr(effect),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathEffectNative_nPath1D(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    path_ptr: jlong,
    advance: jfloat,
    phase: jfloat,
    style: jint,
) -> jlong {
    let path = unsafe { borrow::<Path>(path_ptr) };
    match PathEffect::path_1d(path, advance, phase, path_1d_style_from_ordinal(style)) {
        Some(effect) => box_ptr(effect),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathEffectNative_nPath2D(env: JNIEnv, _class: jni::objects::JClass, matrix: jfloatArray, path_ptr: jlong) -> jlong {
    let array = unsafe { jni::objects::JFloatArray::from_raw(matrix) };
    let mut values = [0f32; 9];
    env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
    let path = unsafe { borrow::<Path>(path_ptr) };
    match PathEffect::path_2d(&Matrix::from_array(values), path) {
        Some(effect) => box_ptr(effect),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathEffectNative_nLine2D(env: JNIEnv, _class: jni::objects::JClass, width: jfloat, matrix: jfloatArray) -> jlong {
    let array = unsafe { jni::objects::JFloatArray::from_raw(matrix) };
    let mut values = [0f32; 9];
    env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
    match PathEffect::line_2d(width, &Matrix::from_array(values)) {
        Some(effect) => box_ptr(effect),
        None => 0,
    }
}
