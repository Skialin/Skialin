use jni::sys::{jfloatArray, jint, jintArray, jlong, jshortArray};
use jni::JNIEnv;

use skialin_core::{Point, VertexMode, Vertices};

use crate::util::drop_ptr;

fn vertex_mode_from_ordinal(ordinal: jint) -> VertexMode {
    match ordinal {
        1 => VertexMode::TriangleStrip,
        2 => VertexMode::TriangleFan,
        _ => VertexMode::Triangles,
    }
}

fn points_from_flat(env: &JNIEnv, array: jfloatArray) -> Vec<Point> {
    if array.is_null() {
        return Vec::new();
    }
    let arr = unsafe { jni::objects::JFloatArray::from_raw(array) };
    let len = env.get_array_length(&arr).expect("get_array_length") as usize;
    let mut buf = vec![0f32; len];
    env.get_float_array_region(&arr, 0, &mut buf).expect("get_float_array_region");
    buf.chunks_exact(2).map(|c| Point::new(c[0], c[1])).collect()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_VerticesNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<Vertices>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_VerticesNative_nMakeCopy(
    env: JNIEnv,
    _class: jni::objects::JClass,
    mode: jint,
    positions: jfloatArray,
    texs: jfloatArray,
    colors: jintArray,
    indices: jshortArray,
) -> jlong {
    let positions = points_from_flat(&env, positions);
    let texs = points_from_flat(&env, texs);

    let colors: Vec<u32> = if colors.is_null() {
        Vec::new()
    } else {
        let arr = unsafe { jni::objects::JIntArray::from_raw(colors) };
        let len = env.get_array_length(&arr).expect("get_array_length") as usize;
        let mut buf = vec![0i32; len];
        env.get_int_array_region(&arr, 0, &mut buf).expect("get_int_array_region");
        buf.into_iter().map(|c| c as u32).collect()
    };

    let indices: Vec<u16> = if indices.is_null() {
        Vec::new()
    } else {
        let arr = unsafe { jni::objects::JShortArray::from_raw(indices) };
        let len = env.get_array_length(&arr).expect("get_array_length") as usize;
        let mut buf = vec![0i16; len];
        env.get_short_array_region(&arr, 0, &mut buf).expect("get_short_array_region");
        buf.into_iter().map(|i| i as u16).collect()
    };

    match Vertices::make_copy(vertex_mode_from_ordinal(mode), &positions, &texs, &colors, &indices) {
        Some(vertices) => crate::util::box_ptr(vertices),
        None => 0,
    }
}
