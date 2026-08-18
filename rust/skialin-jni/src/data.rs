use jni::objects::{JByteArray, JClass, JString};
use jni::sys::{jboolean, jbyteArray, jlong, jobject};
use jni::JNIEnv;

use skialin_core::Data;

use crate::util::{borrow, box_ptr, drop_ptr};

#[no_mangle]
pub extern "system" fn Java_org_skialin_DataNative_nMakeEmpty(_env: JNIEnv, _class: JClass) -> jlong {
    box_ptr(Data::empty())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DataNative_nMakeWithCopy(env: JNIEnv, _class: JClass, bytes: jbyteArray) -> jlong {
    let bytes = unsafe { JByteArray::from_raw(bytes) };
    let len = env.get_array_length(&bytes).expect("get_array_length") as usize;
    let mut buf = vec![0i8; len];
    env.get_byte_array_region(&bytes, 0, &mut buf).expect("get_byte_array_region");
    let buf: Vec<u8> = buf.into_iter().map(|b| b as u8).collect();
    box_ptr(Data::with_copy(&buf))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DataNative_nMakeUninitialized(_env: JNIEnv, _class: JClass, length: jlong) -> jlong {
    box_ptr(Data::uninitialized(length as usize))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DataNative_nMakeZeroInitialized(_env: JNIEnv, _class: JClass, length: jlong) -> jlong {
    box_ptr(Data::zero_initialized(length as usize))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DataNative_nMakeFromFileName(mut env: JNIEnv, _class: JClass, path: JString) -> jlong {
    let path: String = env.get_string(&path).expect("get_string").into();
    match Data::from_file(&path) {
        Some(data) => box_ptr(data),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DataNative_nRelease(_env: JNIEnv, _class: JClass, ptr: jlong) {
    unsafe { drop_ptr::<Data>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DataNative_nSize(_env: JNIEnv, _class: JClass, ptr: jlong) -> jlong {
    unsafe { borrow::<Data>(ptr) }.size() as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DataNative_nIsEmpty(_env: JNIEnv, _class: JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Data>(ptr) }.is_empty() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DataNative_nBytes(env: JNIEnv, _class: JClass, ptr: jlong) -> jbyteArray {
    let data = unsafe { borrow::<Data>(ptr) };
    let bytes = data.as_bytes();
    let array = env.new_byte_array(bytes.len() as i32).expect("new_byte_array");
    let signed: &[i8] = unsafe { std::slice::from_raw_parts(bytes.as_ptr().cast(), bytes.len()) };
    env.set_byte_array_region(&array, 0, signed).expect("set_byte_array_region");
    array.into_raw()
}

/// Zero-copy view over the data's backing storage. Valid only for the
/// lifetime of the owning `Data`.
#[no_mangle]
pub extern "system" fn Java_org_skialin_DataNative_nByteBuffer(mut env: JNIEnv, _class: JClass, ptr: jlong) -> jobject {
    let data = unsafe { borrow::<Data>(ptr) };
    let bytes = data.as_bytes();
    let buffer = unsafe { env.new_direct_byte_buffer(bytes.as_ptr() as *mut u8, bytes.len()) }.expect("new_direct_byte_buffer");
    buffer.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DataNative_nCopyRange(env: JNIEnv, _class: JClass, ptr: jlong, offset: jlong, length: jlong) -> jbyteArray {
    let data = unsafe { borrow::<Data>(ptr) };
    let bytes = data.copy_range(offset as usize, length as usize);
    let array = env.new_byte_array(bytes.len() as i32).expect("new_byte_array");
    let signed: Vec<i8> = bytes.into_iter().map(|b| b as i8).collect();
    env.set_byte_array_region(&array, 0, &signed).expect("set_byte_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DataNative_nCopySubset(_env: JNIEnv, _class: JClass, ptr: jlong, offset: jlong, length: jlong) -> jlong {
    match unsafe { borrow::<Data>(ptr) }.copy_subset(offset as usize, length as usize) {
        Some(subset) => box_ptr(subset),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DataNative_nShareSubset(_env: JNIEnv, _class: JClass, ptr: jlong, offset: jlong, length: jlong) -> jlong {
    match unsafe { borrow::<Data>(ptr) }.share_subset(offset as usize, length as usize) {
        Some(subset) => box_ptr(subset),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DataNative_nEquals(_env: JNIEnv, _class: JClass, ptr: jlong, other_ptr: jlong) -> jboolean {
    let data = unsafe { borrow::<Data>(ptr) };
    let other = unsafe { borrow::<Data>(other_ptr) };
    data.equals(other) as jboolean
}
