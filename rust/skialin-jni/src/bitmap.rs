use jni::sys::{jint, jlong};
use jni::JNIEnv;

use skialin_core::{Bitmap, ImageInfo};

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};

#[no_mangle]
pub extern "system" fn Java_org_skialin_BitmapNative_nMake(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    box_ptr(Bitmap::new())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BitmapNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<Bitmap>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BitmapNative_nAllocPixels(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, info_ptr: jlong) {
    let info = unsafe { borrow::<ImageInfo>(info_ptr) };
    unsafe { borrow_mut::<Bitmap>(ptr) }.alloc_pixels(info);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BitmapNative_nWidth(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Bitmap>(ptr) }.width()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BitmapNative_nHeight(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Bitmap>(ptr) }.height()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BitmapNative_nRowBytes(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    unsafe { borrow::<Bitmap>(ptr) }.row_bytes() as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BitmapNative_nEraseColor(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, color: jint) {
    unsafe { borrow_mut::<Bitmap>(ptr) }.erase_color(color as u32);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BitmapNative_nReadPixels(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jni::sys::jbyteArray {
    let pixels = unsafe { borrow_mut::<Bitmap>(ptr) }.pixels();
    let array = env.new_byte_array(pixels.len() as i32).expect("new_byte_array");
    let signed: &[i8] = unsafe { std::slice::from_raw_parts(pixels.as_ptr() as *const i8, pixels.len()) };
    env.set_byte_array_region(&array, 0, signed).expect("set_byte_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BitmapNative_nAsImage(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    match unsafe { borrow::<Bitmap>(ptr) }.as_image() {
        Some(image) => box_ptr(image),
        None => 0,
    }
}
