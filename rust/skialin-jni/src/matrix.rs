use jni::sys::{jfloat, jfloatArray};
use jni::JNIEnv;

use skialin_core::{Matrix, Point, Rect};

fn read_matrix(env: &JNIEnv, array: jfloatArray) -> Matrix {
    let array = unsafe { jni::objects::JFloatArray::from_raw(array) };
    let mut values = [0f32; 9];
    env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
    Matrix::from_array(values)
}

fn write_floats(env: &JNIEnv, values: &[f32]) -> jfloatArray {
    let array = env.new_float_array(values.len() as i32).expect("new_float_array");
    env.set_float_array_region(&array, 0, values).expect("set_float_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_MatrixNative_nIdentity(env: JNIEnv, _class: jni::objects::JClass) -> jfloatArray {
    write_floats(&env, &Matrix::identity().to_array())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_MatrixNative_nTranslate(env: JNIEnv, _class: jni::objects::JClass, dx: jfloat, dy: jfloat) -> jfloatArray {
    write_floats(&env, &Matrix::translate(dx, dy).to_array())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_MatrixNative_nScale(env: JNIEnv, _class: jni::objects::JClass, sx: jfloat, sy: jfloat) -> jfloatArray {
    write_floats(&env, &Matrix::scale(sx, sy).to_array())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_MatrixNative_nRotate(env: JNIEnv, _class: jni::objects::JClass, degrees: jfloat) -> jfloatArray {
    write_floats(&env, &Matrix::rotate(degrees).to_array())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_MatrixNative_nConcat(env: JNIEnv, _class: jni::objects::JClass, a: jfloatArray, b: jfloatArray) -> jfloatArray {
    let a = read_matrix(&env, a);
    let b = read_matrix(&env, b);
    write_floats(&env, &Matrix::concat(&a, &b).to_array())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_MatrixNative_nInvert(env: JNIEnv, _class: jni::objects::JClass, m: jfloatArray) -> jfloatArray {
    match read_matrix(&env, m).invert() {
        Some(inverted) => write_floats(&env, &inverted.to_array()),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_MatrixNative_nMapPoint(env: JNIEnv, _class: jni::objects::JClass, m: jfloatArray, x: jfloat, y: jfloat) -> jfloatArray {
    let mapped = read_matrix(&env, m).map_point(Point::new(x, y));
    write_floats(&env, &[mapped.x, mapped.y])
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_MatrixNative_nMapRect(
    env: JNIEnv,
    _class: jni::objects::JClass,
    m: jfloatArray,
    left: jfloat,
    top: jfloat,
    right: jfloat,
    bottom: jfloat,
) -> jfloatArray {
    let mapped = read_matrix(&env, m).map_rect(Rect::new(left, top, right, bottom));
    write_floats(&env, &[mapped.left, mapped.top, mapped.right, mapped.bottom])
}
