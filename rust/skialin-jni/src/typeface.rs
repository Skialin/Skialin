use jni::sys::{jboolean, jint, jlong, jstring};
use jni::JNIEnv;

use skialin_core::Typeface;

use crate::util::{borrow, box_ptr, drop_ptr};

#[no_mangle]
pub extern "system" fn Java_org_skialin_TypefaceNative_nMakeEmpty(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    box_ptr(Typeface::empty())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TypefaceNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<Typeface>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TypefaceNative_nUniqueId(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Typeface>(ptr) }.unique_id() as jint
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TypefaceNative_nIsBold(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Typeface>(ptr) }.is_bold() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TypefaceNative_nIsItalic(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Typeface>(ptr) }.is_italic() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TypefaceNative_nIsFixedPitch(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Typeface>(ptr) }.is_fixed_pitch() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TypefaceNative_nCountGlyphs(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Typeface>(ptr) }.count_glyphs()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TypefaceNative_nUnitsPerEm(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Typeface>(ptr) }.units_per_em()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TypefaceNative_nUnicharToGlyph(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, unichar: jint) -> jint {
    unsafe { borrow::<Typeface>(ptr) }.unichar_to_glyph(unichar) as jint
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TypefaceNative_nWeight(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Typeface>(ptr) }.font_style().weight
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TypefaceNative_nWidth(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Typeface>(ptr) }.font_style().width
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TypefaceNative_nSlant(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    i32::from(unsafe { borrow::<Typeface>(ptr) }.font_style().slant)
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TypefaceNative_nFamilyName<'l>(env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, ptr: jlong) -> jstring {
    let name = unsafe { borrow::<Typeface>(ptr) }.family_name();
    env.new_string(name).expect("new_string").into_raw()
}
