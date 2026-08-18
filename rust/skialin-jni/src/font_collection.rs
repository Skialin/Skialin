use jni::sys::jlong;
use jni::JNIEnv;

use skialin_core::{FontCollection, FontMgr};

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontCollectionNative_nNew(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    box_ptr(FontCollection::new())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontCollectionNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<FontCollection>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontCollectionNative_nSetDefaultFontManager(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, font_mgr_ptr: jlong) {
    let font_mgr = unsafe { borrow::<FontMgr>(font_mgr_ptr) };
    unsafe { borrow_mut::<FontCollection>(ptr) }.set_default_font_manager(font_mgr);
}
