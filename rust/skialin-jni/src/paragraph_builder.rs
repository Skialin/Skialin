use jni::sys::jlong;
use jni::JNIEnv;

use skialin_core::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle};

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphBuilderNative_nNew(_env: JNIEnv, _class: jni::objects::JClass, style_ptr: jlong, font_collection_ptr: jlong) -> jlong {
    let style = unsafe { borrow::<ParagraphStyle>(style_ptr) };
    let font_collection = unsafe { borrow_mut::<FontCollection>(font_collection_ptr) };
    box_ptr(ParagraphBuilder::new(style, font_collection))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphBuilderNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<ParagraphBuilder>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphBuilderNative_nPushStyle(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, style_ptr: jlong) {
    let style = unsafe { borrow::<TextStyle>(style_ptr) };
    unsafe { borrow_mut::<ParagraphBuilder>(ptr) }.push_style(style);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphBuilderNative_nPop(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { borrow_mut::<ParagraphBuilder>(ptr) }.pop();
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphBuilderNative_nAddText<'l>(mut env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, ptr: jlong, text: jni::objects::JString<'l>) {
    let text: String = env.get_string(&text).expect("get_string").into();
    unsafe { borrow_mut::<ParagraphBuilder>(ptr) }.add_text(&text);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphBuilderNative_nBuild(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    box_ptr(unsafe { borrow_mut::<ParagraphBuilder>(ptr) }.build())
}
