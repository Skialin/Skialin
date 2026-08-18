use jni::sys::{jboolean, jfloat, jfloatArray, jlong};
use jni::JNIEnv;

use skialin_core::M44;

use crate::util::{borrow, box_ptr, drop_ptr};

#[no_mangle]
pub extern "system" fn Java_org_skialin_M44Native_nMakeIdentity(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    box_ptr(M44::identity())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_M44Native_nMakeFromRowMajor(env: JNIEnv, _class: jni::objects::JClass, row_major: jfloatArray) -> jlong {
    let array = unsafe { jni::objects::JFloatArray::from_raw(row_major) };
    let mut values = [0f32; 16];
    env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
    box_ptr(M44::from_row_major(&values))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_M44Native_nMakeTranslate(_env: JNIEnv, _class: jni::objects::JClass, x: jfloat, y: jfloat, z: jfloat) -> jlong {
    box_ptr(M44::translate(x, y, z))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_M44Native_nMakeScale(_env: JNIEnv, _class: jni::objects::JClass, x: jfloat, y: jfloat, z: jfloat) -> jlong {
    box_ptr(M44::scale(x, y, z))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_M44Native_nMakeRotate(_env: JNIEnv, _class: jni::objects::JClass, axis_x: jfloat, axis_y: jfloat, axis_z: jfloat, radians: jfloat) -> jlong {
    box_ptr(M44::rotate((axis_x, axis_y, axis_z), radians))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_M44Native_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<M44>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_M44Native_nClone(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    box_ptr(unsafe { borrow::<M44>(ptr) }.clone())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_M44Native_nRowMajor(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloatArray {
    let values = unsafe { borrow::<M44>(ptr) }.to_row_major();
    let array = env.new_float_array(16).expect("new_float_array");
    env.set_float_array_region(&array, 0, &values).expect("set_float_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_M44Native_nConcat(_env: JNIEnv, _class: jni::objects::JClass, a_ptr: jlong, b_ptr: jlong) -> jlong {
    let a = unsafe { borrow::<M44>(a_ptr) };
    let b = unsafe { borrow::<M44>(b_ptr) };
    box_ptr(M44::concat(a, b))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_M44Native_nInvert(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    match unsafe { borrow::<M44>(ptr) }.invert() {
        Some(inv) => box_ptr(inv),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_M44Native_nMap(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, v: jfloatArray) -> jfloatArray {
    let array = unsafe { jni::objects::JFloatArray::from_raw(v) };
    let mut values = [0f32; 4];
    env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
    let out = unsafe { borrow::<M44>(ptr) }.map(values);
    let out_array = env.new_float_array(4).expect("new_float_array");
    env.set_float_array_region(&out_array, 0, &out).expect("set_float_array_region");
    out_array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_M44Native_nEquals(_env: JNIEnv, _class: jni::objects::JClass, a_ptr: jlong, b_ptr: jlong) -> jboolean {
    let a = unsafe { borrow::<M44>(a_ptr) };
    let b = unsafe { borrow::<M44>(b_ptr) };
    (a == b) as jboolean
}
