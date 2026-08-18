use jni::sys::{jboolean, jfloatArray, jint, jintArray, jlong};
use jni::JNIEnv;

use skialin_core::{Matrix, Point, Shader, TileMode};

use crate::util::{borrow, box_ptr, drop_ptr};

fn tile_mode_from_ordinal(ordinal: jint) -> TileMode {
    match ordinal {
        1 => TileMode::Repeat,
        2 => TileMode::Mirror,
        3 => TileMode::Decal,
        _ => TileMode::Clamp,
    }
}

fn colors_from_array(env: &JNIEnv, colors: jintArray) -> Vec<u32> {
    let array = unsafe { jni::objects::JIntArray::from_raw(colors) };
    let len = env.get_array_length(&array).expect("get_array_length") as usize;
    let mut buf = vec![0i32; len];
    env.get_int_array_region(&array, 0, &mut buf).expect("get_int_array_region");
    buf.into_iter().map(|c| c as u32).collect()
}

fn positions_from_nullable_array(env: &JNIEnv, positions: jfloatArray) -> Option<Vec<f32>> {
    if positions.is_null() {
        return None;
    }
    let array = unsafe { jni::objects::JFloatArray::from_raw(positions) };
    let len = env.get_array_length(&array).expect("get_array_length") as usize;
    let mut buf = vec![0f32; len];
    env.get_float_array_region(&array, 0, &mut buf).expect("get_float_array_region");
    Some(buf)
}

fn matrix_from_nullable_array(env: &JNIEnv, array: jfloatArray) -> Option<Matrix> {
    if array.is_null() {
        return None;
    }
    let array = unsafe { jni::objects::JFloatArray::from_raw(array) };
    let mut values = [0f32; 9];
    env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
    Some(Matrix::from_array(values))
}

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

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_ShaderNative_nMakeLinearGradient(
    env: JNIEnv,
    _class: jni::objects::JClass,
    x0: jni::sys::jfloat,
    y0: jni::sys::jfloat,
    x1: jni::sys::jfloat,
    y1: jni::sys::jfloat,
    colors: jintArray,
    positions: jfloatArray,
    tile_mode: jint,
    local_matrix: jfloatArray,
) -> jlong {
    let colors = colors_from_array(&env, colors);
    let positions = positions_from_nullable_array(&env, positions);
    let matrix = matrix_from_nullable_array(&env, local_matrix);
    match Shader::linear_gradient([Point::new(x0, y0), Point::new(x1, y1)], &colors, positions.as_deref(), tile_mode_from_ordinal(tile_mode), matrix.as_ref()) {
        Some(shader) => box_ptr(shader),
        None => 0,
    }
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_ShaderNative_nMakeRadialGradient(
    env: JNIEnv,
    _class: jni::objects::JClass,
    cx: jni::sys::jfloat,
    cy: jni::sys::jfloat,
    radius: jni::sys::jfloat,
    colors: jintArray,
    positions: jfloatArray,
    tile_mode: jint,
    local_matrix: jfloatArray,
) -> jlong {
    let colors = colors_from_array(&env, colors);
    let positions = positions_from_nullable_array(&env, positions);
    let matrix = matrix_from_nullable_array(&env, local_matrix);
    match Shader::radial_gradient(Point::new(cx, cy), radius, &colors, positions.as_deref(), tile_mode_from_ordinal(tile_mode), matrix.as_ref()) {
        Some(shader) => box_ptr(shader),
        None => 0,
    }
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_ShaderNative_nMakeTwoPointConicalGradient(
    env: JNIEnv,
    _class: jni::objects::JClass,
    start_x: jni::sys::jfloat,
    start_y: jni::sys::jfloat,
    start_radius: jni::sys::jfloat,
    end_x: jni::sys::jfloat,
    end_y: jni::sys::jfloat,
    end_radius: jni::sys::jfloat,
    colors: jintArray,
    positions: jfloatArray,
    tile_mode: jint,
    local_matrix: jfloatArray,
) -> jlong {
    let colors = colors_from_array(&env, colors);
    let positions = positions_from_nullable_array(&env, positions);
    let matrix = matrix_from_nullable_array(&env, local_matrix);
    match Shader::two_point_conical_gradient(
        Point::new(start_x, start_y),
        start_radius,
        Point::new(end_x, end_y),
        end_radius,
        &colors,
        positions.as_deref(),
        tile_mode_from_ordinal(tile_mode),
        matrix.as_ref(),
    ) {
        Some(shader) => box_ptr(shader),
        None => 0,
    }
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_ShaderNative_nMakeSweepGradient(
    env: JNIEnv,
    _class: jni::objects::JClass,
    cx: jni::sys::jfloat,
    cy: jni::sys::jfloat,
    start_angle: jni::sys::jfloat,
    end_angle: jni::sys::jfloat,
    colors: jintArray,
    positions: jfloatArray,
    tile_mode: jint,
    local_matrix: jfloatArray,
) -> jlong {
    let colors = colors_from_array(&env, colors);
    let positions = positions_from_nullable_array(&env, positions);
    let matrix = matrix_from_nullable_array(&env, local_matrix);
    match Shader::sweep_gradient(Point::new(cx, cy), start_angle, end_angle, &colors, positions.as_deref(), tile_mode_from_ordinal(tile_mode), matrix.as_ref()) {
        Some(shader) => box_ptr(shader),
        None => 0,
    }
}
