use jni::sys::{jboolean, jbyteArray, jint, jintArray, jlong};
use jni::JNIEnv;

use skialin_core::{Codec, ImageInfo};

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};

#[no_mangle]
pub extern "system" fn Java_org_skialin_CodecNative_nMakeFromBytes(env: JNIEnv, _class: jni::objects::JClass, bytes: jbyteArray) -> jlong {
    let array = unsafe { jni::objects::JByteArray::from_raw(bytes) };
    let len = env.get_array_length(&array).expect("get_array_length") as usize;
    let mut buf = vec![0i8; len];
    env.get_byte_array_region(&array, 0, &mut buf).expect("get_byte_array_region");
    let bytes: Vec<u8> = buf.into_iter().map(|b| b as u8).collect();
    match Codec::from_bytes(&bytes) {
        Some(codec) => box_ptr(codec),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CodecNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<Codec>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CodecNative_nDimensions(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jintArray {
    let (width, height) = unsafe { borrow::<Codec>(ptr) }.dimensions();
    let array = env.new_int_array(2).expect("new_int_array");
    env.set_int_array_region(&array, 0, &[width, height]).expect("set_int_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CodecNative_nGetEncodedFormat(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Codec>(ptr) }.encoded_format()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CodecNative_nGetFrameCount(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Codec>(ptr) }.frame_count()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CodecNative_nGetFrameInfo(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, index: jint) -> jintArray {
    match unsafe { borrow::<Codec>(ptr) }.frame_info(index) {
        Some(info) => {
            let array = env.new_int_array(3).expect("new_int_array");
            let values = [info.duration_ms, info.required_frame.unwrap_or(-1), info.fully_received as i32];
            env.set_int_array_region(&array, 0, &values).expect("set_int_array_region");
            array.into_raw()
        }
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CodecNative_nBufferAddress(env: JNIEnv, _class: jni::objects::JClass, buffer: jni::sys::jobject) -> jlong {
    let buffer = unsafe { jni::objects::JByteBuffer::from_raw(buffer) };
    env.get_direct_buffer_address(&buffer).expect("get_direct_buffer_address") as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CodecNative_nGetPixels(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    dst_info_ptr: jlong,
    dst_addr: jlong,
    dst_row_bytes: jlong,
    frame_index: jint,
) -> jboolean {
    let info = unsafe { borrow::<ImageInfo>(dst_info_ptr) };
    unsafe { borrow_mut::<Codec>(ptr).get_pixels(info, dst_addr as *mut u8, dst_row_bytes as usize, frame_index) as jboolean }
}
