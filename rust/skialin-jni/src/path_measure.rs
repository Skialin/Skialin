use jni::sys::{jboolean, jfloat, jfloatArray, jlong};
use jni::JNIEnv;

use skialin_core::{Path, PathBuilder, PathMeasure};

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathMeasureNative_nNew(_env: JNIEnv, _class: jni::objects::JClass, path_ptr: jlong, force_closed: jboolean, res_scale: jfloat) -> jlong {
    let path = unsafe { borrow::<Path>(path_ptr) };
    box_ptr(PathMeasure::new(path, force_closed != 0, res_scale))
}

/// A path measure with no path attached; call `setPath` before using it.
#[no_mangle]
pub extern "system" fn Java_org_skialin_PathMeasureNative_nNewEmpty(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    box_ptr(PathMeasure::empty())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathMeasureNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<PathMeasure>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathMeasureNative_nSetPath(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, path_ptr: jlong, force_closed: jboolean) {
    let path = (path_ptr != 0).then(|| unsafe { borrow::<Path>(path_ptr) });
    unsafe { borrow_mut::<PathMeasure>(ptr) }.set_path(path, force_closed != 0);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathMeasureNative_nLength(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow_mut::<PathMeasure>(ptr) }.length()
}

/// Writes `[posX, posY, tanX, tanY]` into `out`, a 4-element float array. Returns false
/// (leaving `out` unset) if there's no path or it's zero-length.
#[no_mangle]
pub extern "system" fn Java_org_skialin_PathMeasureNative_nPosTan(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, distance: jfloat, out: jfloatArray) -> jboolean {
    match unsafe { borrow_mut::<PathMeasure>(ptr) }.pos_tan(distance) {
        Some(pos_tan) => {
            let values = [pos_tan.position.x, pos_tan.position.y, pos_tan.tangent.x, pos_tan.tangent.y];
            let array = unsafe { jni::objects::JFloatArray::from_raw(out) };
            env.set_float_array_region(&array, 0, &values).expect("set_float_array_region");
            true as jboolean
        }
        None => false as jboolean,
    }
}

/// Writes the 3x3 matrix into `out`, a 9-element float array. Returns false (leaving
/// `out` unset) if there's no path or it's zero-length.
#[no_mangle]
pub extern "system" fn Java_org_skialin_PathMeasureNative_nMatrix(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, distance: jfloat, out: jfloatArray) -> jboolean {
    match unsafe { borrow_mut::<PathMeasure>(ptr) }.matrix(distance) {
        Some(matrix) => {
            let array = unsafe { jni::objects::JFloatArray::from_raw(out) };
            env.set_float_array_region(&array, 0, &matrix.to_array()).expect("set_float_array_region");
            true as jboolean
        }
        None => false as jboolean,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathMeasureNative_nSegment(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    start_d: jfloat,
    stop_d: jfloat,
    dst_ptr: jlong,
    start_with_move_to: jboolean,
) -> jboolean {
    let dst = unsafe { borrow_mut::<PathBuilder>(dst_ptr) };
    unsafe { borrow_mut::<PathMeasure>(ptr) }.segment(start_d, stop_d, dst, start_with_move_to != 0) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathMeasureNative_nIsClosed(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow_mut::<PathMeasure>(ptr) }.is_closed() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathMeasureNative_nNextContour(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow_mut::<PathMeasure>(ptr) }.next_contour() as jboolean
}
