use jni::sys::{jfloat, jfloatArray};
use jni::JNIEnv;

use skialin_core::ColorMatrix;

fn read20(env: &JNIEnv, array: jfloatArray) -> [f32; 20] {
    let arr = unsafe { jni::objects::JFloatArray::from_raw(array) };
    let mut values = [0f32; 20];
    env.get_float_array_region(&arr, 0, &mut values).expect("get_float_array_region");
    values
}

fn write20(env: &JNIEnv, values: &[f32; 20]) -> jfloatArray {
    let array = env.new_float_array(20).expect("new_float_array");
    env.set_float_array_region(&array, 0, values).expect("set_float_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorMatrixNative_nIdentity(env: JNIEnv, _class: jni::objects::JClass) -> jfloatArray {
    write20(&env, &ColorMatrix::identity().0)
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorMatrixNative_nScale(env: JNIEnv, _class: jni::objects::JClass, r: jfloat, g: jfloat, b: jfloat, a: jfloat) -> jfloatArray {
    write20(&env, &ColorMatrix::scale(r, g, b, a).0)
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorMatrixNative_nSaturation(env: JNIEnv, _class: jni::objects::JClass, sat: jfloat) -> jfloatArray {
    write20(&env, &ColorMatrix::saturation(sat).0)
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorMatrixNative_nPostTranslate(
    env: JNIEnv,
    _class: jni::objects::JClass,
    mat20: jfloatArray,
    dr: jfloat,
    dg: jfloat,
    db: jfloat,
    da: jfloat,
) -> jfloatArray {
    let mut m = ColorMatrix(read20(&env, mat20));
    m.post_translate(dr, dg, db, da);
    write20(&env, &m.0)
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorMatrixNative_nConcat(env: JNIEnv, _class: jni::objects::JClass, a20: jfloatArray, b20: jfloatArray) -> jfloatArray {
    let a = ColorMatrix(read20(&env, a20));
    let b = ColorMatrix(read20(&env, b20));
    write20(&env, &ColorMatrix::concat(&a, &b).0)
}
