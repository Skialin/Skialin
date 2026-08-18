use jni::sys::{jfloat, jint, jlong, jstring};
use jni::JNIEnv;

use skialin_core::{ParagraphStyle, TextAlign, TextDirection, TextHeightBehavior, TextStyle};

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphStyleNative_nNew(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    box_ptr(ParagraphStyle::new())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphStyleNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<ParagraphStyle>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphStyleNative_nTextDirection(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<ParagraphStyle>(ptr) }.text_direction().into()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphStyleNative_nSetTextDirection(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, direction: jint) {
    unsafe { borrow_mut::<ParagraphStyle>(ptr) }.set_text_direction(TextDirection::from(direction));
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphStyleNative_nTextAlign(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<ParagraphStyle>(ptr) }.text_align().into()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphStyleNative_nSetTextAlign(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, align: jint) {
    unsafe { borrow_mut::<ParagraphStyle>(ptr) }.set_text_align(TextAlign::from(align));
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphStyleNative_nMaxLines(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    unsafe { borrow::<ParagraphStyle>(ptr) }.max_lines() as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphStyleNative_nSetMaxLines(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, max_lines: jlong) {
    unsafe { borrow_mut::<ParagraphStyle>(ptr) }.set_max_lines(max_lines as usize);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphStyleNative_nEllipsis<'l>(env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, ptr: jlong) -> jstring {
    let ellipsis = unsafe { borrow::<ParagraphStyle>(ptr) }.ellipsis();
    env.new_string(ellipsis).expect("new_string").into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphStyleNative_nSetEllipsis<'l>(mut env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, ptr: jlong, ellipsis: jni::objects::JString<'l>) {
    let ellipsis: String = env.get_string(&ellipsis).expect("get_string").into();
    unsafe { borrow_mut::<ParagraphStyle>(ptr) }.set_ellipsis(&ellipsis);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphStyleNative_nHeight(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<ParagraphStyle>(ptr) }.height()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphStyleNative_nSetHeight(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, height: jfloat) {
    unsafe { borrow_mut::<ParagraphStyle>(ptr) }.set_height(height);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphStyleNative_nTextHeightBehavior(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<ParagraphStyle>(ptr) }.text_height_behavior().0
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphStyleNative_nSetTextHeightBehavior(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, behavior: jint) {
    unsafe { borrow_mut::<ParagraphStyle>(ptr) }.set_text_height_behavior(TextHeightBehavior(behavior));
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphStyleNative_nTextStyle(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    box_ptr(unsafe { borrow::<ParagraphStyle>(ptr) }.text_style())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphStyleNative_nSetTextStyle(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, style_ptr: jlong) {
    let style = unsafe { borrow::<TextStyle>(style_ptr) };
    unsafe { borrow_mut::<ParagraphStyle>(ptr) }.set_text_style(style);
}
