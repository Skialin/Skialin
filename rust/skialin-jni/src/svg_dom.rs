use jni::sys::{jbyteArray, jfloat, jfloatArray, jlong};
use jni::JNIEnv;

use skialin_core::{Canvas, SVGDOM};

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};

#[no_mangle]
pub extern "system" fn Java_org_skialin_SVGDOMNative_nMakeFromBytes(env: JNIEnv, _class: jni::objects::JClass, bytes: jbyteArray) -> jlong {
    let array = unsafe { jni::objects::JByteArray::from_raw(bytes) };
    let len = env.get_array_length(&array).expect("get_array_length") as usize;
    let mut buf = vec![0i8; len];
    env.get_byte_array_region(&array, 0, &mut buf).expect("get_byte_array_region");
    let bytes: Vec<u8> = buf.into_iter().map(|b| b as u8).collect();
    match SVGDOM::from_bytes(&bytes) {
        Some(dom) => box_ptr(dom),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SVGDOMNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<SVGDOM>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SVGDOMNative_nSetContainerSize(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, width: jfloat, height: jfloat) {
    unsafe { borrow_mut::<SVGDOM>(ptr) }.set_container_size(width, height);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SVGDOMNative_nGetContainerSize(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloatArray {
    let (width, height) = unsafe { borrow::<SVGDOM>(ptr) }.container_size();
    let array = env.new_float_array(2).expect("new_float_array");
    env.set_float_array_region(&array, 0, &[width, height]).expect("set_float_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SVGDOMNative_nSetSizeAndStretch(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, width: jfloat, height: jfloat) {
    unsafe { borrow_mut::<SVGDOM>(ptr) }.set_size_and_stretch(width, height);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SVGDOMNative_nRender(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, canvas_ptr: jlong) {
    let mut canvas = unsafe { Canvas::from_raw(canvas_ptr as *mut skialin_core::sys::SkCanvas) };
    unsafe { borrow::<SVGDOM>(ptr) }.render(&mut canvas);
}
