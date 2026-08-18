use jni::sys::{jbyteArray, jlong, jlongArray};
use jni::JNIEnv;

use skialin_core::{ColorFilter, Matrix, RuntimeEffect, Shader};

use crate::util::{borrow, box_ptr, drop_ptr};

fn bytes_from_nullable_array(env: &JNIEnv, bytes: jbyteArray) -> Vec<u8> {
    if bytes.is_null() {
        return Vec::new();
    }
    let array = unsafe { jni::objects::JByteArray::from_raw(bytes) };
    let len = env.get_array_length(&array).expect("get_array_length") as usize;
    let mut buf = vec![0i8; len];
    env.get_byte_array_region(&array, 0, &mut buf).expect("get_byte_array_region");
    buf.into_iter().map(|b| b as u8).collect()
}

fn matrix_from_nullable_array(env: &JNIEnv, array: jni::sys::jfloatArray) -> Option<Matrix> {
    if array.is_null() {
        return None;
    }
    let array = unsafe { jni::objects::JFloatArray::from_raw(array) };
    let mut values = [0f32; 9];
    env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
    Some(Matrix::from_array(values))
}

/// Boxes `Ok(effect)` as a positive pointer, or throws an IllegalArgumentException
/// with the SkSL compiler's error text and returns 0.
fn finish_make<'l>(env: &mut JNIEnv<'l>, result: Result<RuntimeEffect, String>) -> jlong {
    match result {
        Ok(effect) => box_ptr(effect),
        Err(message) => {
            let _ = env.throw_new("java/lang/IllegalArgumentException", message);
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RuntimeEffectNative_nMakeForShader<'l>(mut env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, sksl: jni::objects::JString<'l>) -> jlong {
    let sksl: String = env.get_string(&sksl).expect("get_string").into();
    let result = RuntimeEffect::make_for_shader(&sksl);
    finish_make(&mut env, result)
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RuntimeEffectNative_nMakeForColorFilter<'l>(mut env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, sksl: jni::objects::JString<'l>) -> jlong {
    let sksl: String = env.get_string(&sksl).expect("get_string").into();
    let result = RuntimeEffect::make_for_color_filter(&sksl);
    finish_make(&mut env, result)
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RuntimeEffectNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<RuntimeEffect>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RuntimeEffectNative_nMakeShader(
    env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    uniforms: jbyteArray,
    children: jlongArray,
    local_matrix: jni::sys::jfloatArray,
) -> jlong {
    let uniforms = bytes_from_nullable_array(&env, uniforms);
    let children_array = unsafe { jni::objects::JLongArray::from_raw(children) };
    let child_count = env.get_array_length(&children_array).expect("get_array_length") as usize;
    let mut child_ptrs = vec![0i64; child_count];
    env.get_long_array_region(&children_array, 0, &mut child_ptrs).expect("get_long_array_region");
    let child_refs: Vec<&Shader> = child_ptrs.iter().map(|&p| unsafe { borrow::<Shader>(p) }).collect();
    let matrix = matrix_from_nullable_array(&env, local_matrix);

    match unsafe { borrow::<RuntimeEffect>(ptr) }.make_shader(&uniforms, &child_refs, matrix.as_ref()) {
        Some(shader) => box_ptr(shader),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RuntimeEffectNative_nMakeColorFilter(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, uniforms: jbyteArray, children: jlongArray) -> jlong {
    let uniforms = bytes_from_nullable_array(&env, uniforms);
    let children_array = unsafe { jni::objects::JLongArray::from_raw(children) };
    let child_count = env.get_array_length(&children_array).expect("get_array_length") as usize;
    let mut child_ptrs = vec![0i64; child_count];
    env.get_long_array_region(&children_array, 0, &mut child_ptrs).expect("get_long_array_region");
    let child_refs: Vec<&ColorFilter> = child_ptrs.iter().map(|&p| unsafe { borrow::<ColorFilter>(p) }).collect();

    match unsafe { borrow::<RuntimeEffect>(ptr) }.make_color_filter(&uniforms, &child_refs) {
        Some(filter) => box_ptr(filter),
        None => 0,
    }
}
