use jni::sys::{jboolean, jfloat, jfloatArray, jint, jlong};
use jni::JNIEnv;

use skialin_core::{ColorFilter, ImageFilter, Matrix, SamplingOptions, TileMode};

use crate::util::{borrow, box_ptr, drop_ptr};

fn tile_mode_from_ordinal(ordinal: jint) -> TileMode {
    match ordinal {
        1 => TileMode::Repeat,
        2 => TileMode::Mirror,
        3 => TileMode::Decal,
        _ => TileMode::Clamp,
    }
}

fn input_from_ptr<'a>(ptr: jlong) -> Option<&'a ImageFilter> {
    (ptr != 0).then(|| unsafe { borrow::<ImageFilter>(ptr) })
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageFilterNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<ImageFilter>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageFilterNative_nBlur(_env: JNIEnv, _class: jni::objects::JClass, sigma_x: jfloat, sigma_y: jfloat, tile_mode: jint, input_ptr: jlong) -> jlong {
    match ImageFilter::blur(sigma_x, sigma_y, tile_mode_from_ordinal(tile_mode), input_from_ptr(input_ptr)) {
        Some(filter) => box_ptr(filter),
        None => 0,
    }
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageFilterNative_nDropShadow(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    dx: jfloat,
    dy: jfloat,
    sigma_x: jfloat,
    sigma_y: jfloat,
    color: jint,
    input_ptr: jlong,
) -> jlong {
    match ImageFilter::drop_shadow(dx, dy, sigma_x, sigma_y, color as u32, input_from_ptr(input_ptr)) {
        Some(filter) => box_ptr(filter),
        None => 0,
    }
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageFilterNative_nDropShadowOnly(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    dx: jfloat,
    dy: jfloat,
    sigma_x: jfloat,
    sigma_y: jfloat,
    color: jint,
    input_ptr: jlong,
) -> jlong {
    match ImageFilter::drop_shadow_only(dx, dy, sigma_x, sigma_y, color as u32, input_from_ptr(input_ptr)) {
        Some(filter) => box_ptr(filter),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageFilterNative_nOffset(_env: JNIEnv, _class: jni::objects::JClass, dx: jfloat, dy: jfloat, input_ptr: jlong) -> jlong {
    match ImageFilter::offset(dx, dy, input_from_ptr(input_ptr)) {
        Some(filter) => box_ptr(filter),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageFilterNative_nColorFilter(_env: JNIEnv, _class: jni::objects::JClass, color_filter_ptr: jlong, input_ptr: jlong) -> jlong {
    let color_filter = unsafe { borrow::<ColorFilter>(color_filter_ptr) };
    match ImageFilter::color_filter(color_filter, input_from_ptr(input_ptr)) {
        Some(filter) => box_ptr(filter),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageFilterNative_nCompose(_env: JNIEnv, _class: jni::objects::JClass, outer_ptr: jlong, inner_ptr: jlong) -> jlong {
    let outer = unsafe { borrow::<ImageFilter>(outer_ptr) };
    let inner = unsafe { borrow::<ImageFilter>(inner_ptr) };
    match ImageFilter::compose(outer, inner) {
        Some(filter) => box_ptr(filter),
        None => 0,
    }
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageFilterNative_nMatrixTransform(
    env: JNIEnv,
    _class: jni::objects::JClass,
    matrix: jfloatArray,
    max_aniso: jint,
    use_cubic: jboolean,
    cubic_b: jfloat,
    cubic_c: jfloat,
    filter: jint,
    mipmap: jint,
    input_ptr: jlong,
) -> jlong {
    let array = unsafe { jni::objects::JFloatArray::from_raw(matrix) };
    let mut values = [0f32; 9];
    env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
    let matrix = Matrix::from_array(values);
    let sampling = SamplingOptions {
        max_aniso,
        cubic: (use_cubic != 0).then_some((cubic_b, cubic_c)),
        filter: if filter == 1 { skialin_core::FilterMode::Linear } else { skialin_core::FilterMode::Nearest },
        mipmap: match mipmap {
            1 => skialin_core::MipmapMode::Nearest,
            2 => skialin_core::MipmapMode::Linear,
            _ => skialin_core::MipmapMode::None,
        },
    };
    match ImageFilter::matrix_transform(&matrix, sampling, input_from_ptr(input_ptr)) {
        Some(filter) => box_ptr(filter),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageFilterNative_nDilate(_env: JNIEnv, _class: jni::objects::JClass, radius_x: jfloat, radius_y: jfloat, input_ptr: jlong) -> jlong {
    match ImageFilter::dilate(radius_x, radius_y, input_from_ptr(input_ptr)) {
        Some(filter) => box_ptr(filter),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageFilterNative_nErode(_env: JNIEnv, _class: jni::objects::JClass, radius_x: jfloat, radius_y: jfloat, input_ptr: jlong) -> jlong {
    match ImageFilter::erode(radius_x, radius_y, input_from_ptr(input_ptr)) {
        Some(filter) => box_ptr(filter),
        None => 0,
    }
}
