use jni::objects::{JByteArray, JClass, JFloatArray};
use jni::sys::{jboolean, jbyteArray, jfloatArray, jint, jlong};
use jni::JNIEnv;

use skialin_core::{CicpPrimaries, CicpTransferFn, ColorSpace};

use crate::util::{borrow, box_ptr, drop_ptr};

fn read_floats<const N: usize>(env: &JNIEnv, array: &JFloatArray) -> [f32; N] {
    let mut out = [0f32; N];
    env.get_float_array_region(array, 0, &mut out).expect("get_float_array_region");
    out
}

fn write_floats(env: &JNIEnv, values: &[f32]) -> jfloatArray {
    let array = env.new_float_array(values.len() as i32).expect("new_float_array");
    env.set_float_array_region(&array, 0, values).expect("set_float_array_region");
    array.into_raw()
}

/// `ptr` is a `ColorSpace` pointer, or `0` for "no color space".
pub(crate) fn color_space_ptr_from_jlong<'a>(ptr: jlong) -> Option<&'a ColorSpace> {
    (ptr != 0).then(|| unsafe { borrow::<ColorSpace>(ptr) })
}

fn cicp_primaries_from_code(v: jint) -> CicpPrimaries {
    use CicpPrimaries::*;
    match v {
        1 => Rec709,
        5 => Rec470SystemBG,
        6 => Rec601,
        7 => SmpteSt240,
        8 => GenericFilm,
        9 => Rec2020,
        10 => SmpteSt428_1,
        11 => SmpteRp431_2,
        12 => SmpteEg432_1,
        _ => Rec470SystemM,
    }
}

fn cicp_transfer_fn_from_code(v: jint) -> CicpTransferFn {
    use CicpTransferFn::*;
    match v {
        1 => Rec709,
        5 => Rec470SystemBG,
        6 => Rec601,
        7 => SmpteSt240,
        8 => Linear,
        11 => Iec61966_2_4,
        13 => Srgb,
        14 => Rec2020_10bit,
        15 => Rec2020_12bit,
        16 => Pq,
        17 => SmpteSt428_1,
        18 => Hlg,
        _ => Rec470SystemM,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nMakeSRGB(_env: JNIEnv, _class: JClass) -> jlong {
    box_ptr(ColorSpace::srgb())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nMakeSRGBLinear(_env: JNIEnv, _class: JClass) -> jlong {
    box_ptr(ColorSpace::srgb_linear())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nMakeRGB(env: JNIEnv, _class: JClass, transfer_fn: jfloatArray, to_xyz_d50: jfloatArray) -> jlong {
    let transfer_fn = unsafe { JFloatArray::from_raw(transfer_fn) };
    let to_xyz_d50 = unsafe { JFloatArray::from_raw(to_xyz_d50) };
    box_ptr(ColorSpace::rgb(read_floats(&env, &transfer_fn), read_floats(&env, &to_xyz_d50)))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nMakeCICP(_env: JNIEnv, _class: JClass, primaries: jint, transfer_fn: jint) -> jlong {
    match ColorSpace::cicp(cicp_primaries_from_code(primaries), cicp_transfer_fn_from_code(transfer_fn)) {
        Some(cs) => box_ptr(cs),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nMakeFromIccProfile(env: JNIEnv, _class: JClass, bytes: jbyteArray) -> jlong {
    let bytes = unsafe { JByteArray::from_raw(bytes) };
    let len = env.get_array_length(&bytes).expect("get_array_length") as usize;
    let mut buf = vec![0i8; len];
    env.get_byte_array_region(&bytes, 0, &mut buf).expect("get_byte_array_region");
    let buf: Vec<u8> = buf.into_iter().map(|b| b as u8).collect();
    match ColorSpace::from_icc_profile(&buf) {
        Some(cs) => box_ptr(cs),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nDeserialize(env: JNIEnv, _class: JClass, bytes: jbyteArray) -> jlong {
    let bytes = unsafe { JByteArray::from_raw(bytes) };
    let len = env.get_array_length(&bytes).expect("get_array_length") as usize;
    let mut buf = vec![0i8; len];
    env.get_byte_array_region(&bytes, 0, &mut buf).expect("get_byte_array_region");
    let buf: Vec<u8> = buf.into_iter().map(|b| b as u8).collect();
    match ColorSpace::deserialize(&buf) {
        Some(cs) => box_ptr(cs),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nRelease(_env: JNIEnv, _class: JClass, ptr: jlong) {
    unsafe { drop_ptr::<ColorSpace>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nGammaCloseToSrgb(_env: JNIEnv, _class: JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<ColorSpace>(ptr) }.gamma_close_to_srgb() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nGammaIsLinear(_env: JNIEnv, _class: JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<ColorSpace>(ptr) }.gamma_is_linear() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nIsSrgb(_env: JNIEnv, _class: JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<ColorSpace>(ptr) }.is_srgb() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nToXyzD50(env: JNIEnv, _class: JClass, ptr: jlong) -> jfloatArray {
    match unsafe { borrow::<ColorSpace>(ptr) }.to_xyz_d50() {
        Some(m) => write_floats(&env, &m),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nToXyzD50Hash(_env: JNIEnv, _class: JClass, ptr: jlong) -> jint {
    unsafe { borrow::<ColorSpace>(ptr) }.to_xyz_d50_hash() as jint
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nTransferFn(env: JNIEnv, _class: JClass, ptr: jlong) -> jfloatArray {
    write_floats(&env, &unsafe { borrow::<ColorSpace>(ptr) }.transfer_fn())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nInvTransferFn(env: JNIEnv, _class: JClass, ptr: jlong) -> jfloatArray {
    write_floats(&env, &unsafe { borrow::<ColorSpace>(ptr) }.inv_transfer_fn())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nNumericalTransferFn(env: JNIEnv, _class: JClass, ptr: jlong) -> jfloatArray {
    match unsafe { borrow::<ColorSpace>(ptr) }.numerical_transfer_fn() {
        Some(fn_) => write_floats(&env, &fn_),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nTransferFnHash(_env: JNIEnv, _class: JClass, ptr: jlong) -> jint {
    unsafe { borrow::<ColorSpace>(ptr) }.transfer_fn_hash() as jint
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nHash(_env: JNIEnv, _class: JClass, ptr: jlong) -> jlong {
    unsafe { borrow::<ColorSpace>(ptr) }.hash() as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nGamutTransformTo(env: JNIEnv, _class: JClass, ptr: jlong, dst_ptr: jlong) -> jfloatArray {
    let src = unsafe { borrow::<ColorSpace>(ptr) };
    let dst = unsafe { borrow::<ColorSpace>(dst_ptr) };
    write_floats(&env, &src.gamut_transform_to(dst))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nMakeLinearGamma(_env: JNIEnv, _class: JClass, ptr: jlong) -> jlong {
    box_ptr(unsafe { borrow::<ColorSpace>(ptr) }.make_linear_gamma())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nMakeSRGBGamma(_env: JNIEnv, _class: JClass, ptr: jlong) -> jlong {
    box_ptr(unsafe { borrow::<ColorSpace>(ptr) }.make_srgb_gamma())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nMakeColorSpin(_env: JNIEnv, _class: JClass, ptr: jlong) -> jlong {
    box_ptr(unsafe { borrow::<ColorSpace>(ptr) }.make_color_spin())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nSerialize(_env: JNIEnv, _class: JClass, ptr: jlong) -> jlong {
    box_ptr(unsafe { borrow::<ColorSpace>(ptr) }.serialize())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ColorSpaceNative_nEquals(_env: JNIEnv, _class: JClass, ptr: jlong, other_ptr: jlong) -> jboolean {
    let a = unsafe { borrow::<ColorSpace>(ptr) };
    let b = unsafe { borrow::<ColorSpace>(other_ptr) };
    a.equals(b) as jboolean
}
