use jni::sys::{jboolean, jlong};
use jni::JNIEnv;

use skialin_core::DirectContext;

use crate::util::{borrow_mut, box_ptr, drop_ptr};

#[no_mangle]
pub extern "system" fn Java_org_skialin_DirectContextNative_nMakeGL(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    match DirectContext::new_gl() {
        Some(context) => box_ptr(context),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DirectContextNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<DirectContext>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DirectContextNative_nFlush(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { borrow_mut::<DirectContext>(ptr) }.flush();
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DirectContextNative_nSubmit(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, sync_cpu: jboolean) {
    unsafe { borrow_mut::<DirectContext>(ptr) }.submit(sync_cpu != 0);
}
