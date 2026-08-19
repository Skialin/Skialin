use jni::sys::{jint, jlong};
use jni::JNIEnv;

use skialin_core::Blender;

use crate::paint::blend_mode_from_ordinal;
use crate::util::drop_ptr;

#[no_mangle]
pub extern "system" fn Java_org_skialin_BlenderNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<Blender>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_BlenderNative_nMode(_env: JNIEnv, _class: jni::objects::JClass, mode: jint) -> jlong {
    crate::util::box_ptr(Blender::mode(blend_mode_from_ordinal(mode)))
}
