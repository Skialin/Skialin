use jni::sys::{jboolean, jfloatArray, jint, jlong};
use jni::JNIEnv;

use skialin_core::{Matrix, Shader};

use crate::util::{borrow, box_ptr, drop_ptr};

#[no_mangle]
pub extern "system" fn Java_org_skialin_ShaderNative_nMakeEmpty(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    box_ptr(Shader::empty())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ShaderNative_nMakeColor(_env: JNIEnv, _class: jni::objects::JClass, color: jint) -> jlong {
    box_ptr(Shader::color(color as u32))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ShaderNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<Shader>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ShaderNative_nIsOpaque(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Shader>(ptr) }.is_opaque() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ShaderNative_nWithLocalMatrix(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, matrix: jfloatArray) -> jlong {
    let matrix = unsafe { jni::objects::JFloatArray::from_raw(matrix) };
    let mut values = [0f32; 9];
    env.get_float_array_region(&matrix, 0, &mut values).expect("get_float_array_region");
    let m = Matrix::from_array(values);
    box_ptr(unsafe { borrow::<Shader>(ptr) }.with_local_matrix(&m))
}
