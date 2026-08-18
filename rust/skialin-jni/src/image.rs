use jni::sys::{jint, jlong};
use jni::JNIEnv;

use skialin_core::Image;

use crate::util::{borrow, box_ptr, drop_ptr};

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<Image>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nWidth(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Image>(ptr) }.width()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nHeight(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Image>(ptr) }.height()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nDecode(env: JNIEnv, _class: jni::objects::JClass, bytes: jni::sys::jbyteArray) -> jlong {
    let bytes = unsafe { jni::objects::JByteArray::from_raw(bytes) };
    let len = env.get_array_length(&bytes).expect("get_array_length") as usize;
    let mut buf = vec![0i8; len];
    env.get_byte_array_region(&bytes, 0, &mut buf).expect("get_byte_array_region");
    let buf: Vec<u8> = buf.into_iter().map(|b| b as u8).collect();
    match Image::decode(&buf) {
        Some(image) => box_ptr(image),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nEncodeToPng(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jni::sys::jbyteArray {
    match unsafe { borrow::<Image>(ptr) }.encode_to_png() {
        Some(bytes) => {
            let array = env.new_byte_array(bytes.len() as i32).expect("new_byte_array");
            let signed: Vec<i8> = bytes.into_iter().map(|b| b as i8).collect();
            env.set_byte_array_region(&array, 0, &signed).expect("set_byte_array_region");
            array.into_raw()
        }
        None => std::ptr::null_mut(),
    }
}
