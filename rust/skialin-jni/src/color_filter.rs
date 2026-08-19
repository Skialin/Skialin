use jni::sys::{jboolean, jbyteArray, jfloat, jfloatArray, jint, jlong};
use jni::JNIEnv;

use skialin_core::ColorFilter;

use crate::paint::blend_mode_from_ordinal;
use crate::util::{borrow, box_ptr, drop_ptr};

fn read_table(env: &JNIEnv, table: jbyteArray) -> Option<[u8; 256]> {
    if table.is_null() {
        return None;
    }
    let array = unsafe { jni::objects::JByteArray::from_raw(table) };
    let mut buf = [0i8; 256];
    env.get_byte_array_region(&array, 0, &mut buf).expect("get_byte_array_region");
    let mut out = [0u8; 256];
    for i in 0..256 {
        out[i] = buf[i] as u8;
    }
    Some(out)
}

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

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorFilterNative_nHSLAMatrix(env: JNIEnv, _class: jni::objects::JClass, row_major_20: jfloatArray) -> jlong {
    let array = unsafe { jni::objects::JFloatArray::from_raw(row_major_20) };
    let mut values = [0f32; 20];
    env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
    match ColorFilter::hsla_matrix(&values) {
        Some(filter) => box_ptr(filter),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorFilterNative_nLinearToSRGBGamma(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    box_ptr(ColorFilter::linear_to_srgb_gamma())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorFilterNative_nSRGBToLinearGamma(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    box_ptr(ColorFilter::srgb_to_linear_gamma())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorFilterNative_nTable(env: JNIEnv, _class: jni::objects::JClass, table: jbyteArray) -> jlong {
    let table = read_table(&env, table).expect("table must not be null");
    box_ptr(ColorFilter::table(&table))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorFilterNative_nTableARGB(env: JNIEnv, _class: jni::objects::JClass, a: jbyteArray, r: jbyteArray, g: jbyteArray, b: jbyteArray) -> jlong {
    let a = read_table(&env, a);
    let r = read_table(&env, r);
    let g = read_table(&env, g);
    let b = read_table(&env, b);
    box_ptr(ColorFilter::table_argb(a.as_ref(), r.as_ref(), g.as_ref(), b.as_ref()))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorFilterNative_nLighting(_env: JNIEnv, _class: jni::objects::JClass, mul: jint, add: jint) -> jlong {
    box_ptr(ColorFilter::lighting(mul as u32, add as u32))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorFilterNative_nHighContrast(_env: JNIEnv, _class: jni::objects::JClass, grayscale: jboolean, invert_style: jint, contrast: jfloat) -> jlong {
    match ColorFilter::high_contrast(grayscale != 0, invert_style, contrast) {
        Some(filter) => box_ptr(filter),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorFilterNative_nLuma(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    box_ptr(ColorFilter::luma())
}
