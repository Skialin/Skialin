use jni::sys::{jbyteArray, jdouble, jfloat, jfloatArray, jlong};
use jni::JNIEnv;

use skialin_core::{Canvas, Rect, SkottieAnimation};

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};

#[no_mangle]
pub extern "system" fn Java_org_skialin_SkottieAnimationNative_nMakeFromBytes(env: JNIEnv, _class: jni::objects::JClass, bytes: jbyteArray) -> jlong {
    let array = unsafe { jni::objects::JByteArray::from_raw(bytes) };
    let len = env.get_array_length(&array).expect("get_array_length") as usize;
    let mut buf = vec![0i8; len];
    env.get_byte_array_region(&array, 0, &mut buf).expect("get_byte_array_region");
    let bytes: Vec<u8> = buf.into_iter().map(|b| b as u8).collect();
    match SkottieAnimation::from_bytes(&bytes) {
        Some(animation) => box_ptr(animation),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SkottieAnimationNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<SkottieAnimation>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SkottieAnimationNative_nRender(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, canvas_ptr: jlong, dst: jfloatArray) {
    let dst_rect = if dst.is_null() {
        None
    } else {
        let array = unsafe { jni::objects::JFloatArray::from_raw(dst) };
        let mut values = [0f32; 4];
        env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
        Some(Rect::new(values[0], values[1], values[2], values[3]))
    };
    let mut canvas = unsafe { Canvas::from_raw(canvas_ptr as *mut skialin_core::sys::SkCanvas) };
    unsafe { borrow::<SkottieAnimation>(ptr) }.render(&mut canvas, dst_rect);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SkottieAnimationNative_nSeek(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, t: jfloat) {
    unsafe { borrow_mut::<SkottieAnimation>(ptr) }.seek(t);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SkottieAnimationNative_nSeekFrame(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, frame: jdouble) {
    unsafe { borrow_mut::<SkottieAnimation>(ptr) }.seek_frame(frame);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SkottieAnimationNative_nDuration(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jdouble {
    unsafe { borrow::<SkottieAnimation>(ptr) }.duration()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SkottieAnimationNative_nFps(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jdouble {
    unsafe { borrow::<SkottieAnimation>(ptr) }.fps()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SkottieAnimationNative_nSize(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloatArray {
    let (width, height) = unsafe { borrow::<SkottieAnimation>(ptr) }.size();
    let array = env.new_float_array(2).expect("new_float_array");
    env.set_float_array_region(&array, 0, &[width, height]).expect("set_float_array_region");
    array.into_raw()
}
