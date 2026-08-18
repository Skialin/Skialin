use jni::sys::{jboolean, jfloat, jint, jlong, jobjectArray};
use jni::JNIEnv;

use skialin_core::{FontStyle, StrutStyle};

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};

#[no_mangle]
pub extern "system" fn Java_org_skialin_StrutStyleNative_nNew(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    box_ptr(StrutStyle::new())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_StrutStyleNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<StrutStyle>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_StrutStyleNative_nFontFamilies<'l>(mut env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, ptr: jlong) -> jobjectArray {
    let families = unsafe { borrow::<StrutStyle>(ptr) }.font_families();
    let string_class = env.find_class("java/lang/String").expect("find_class");
    let array = env.new_object_array(families.len() as i32, string_class, unsafe { jni::objects::JObject::from_raw(std::ptr::null_mut()) }).expect("new_object_array");
    for (i, family) in families.iter().enumerate() {
        let jstr = env.new_string(family).expect("new_string");
        env.set_object_array_element(&array, i as i32, jstr).expect("set_object_array_element");
    }
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_StrutStyleNative_nSetFontFamilies<'l>(mut env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, ptr: jlong, families: jobjectArray) {
    let array = unsafe { jni::objects::JObjectArray::from_raw(families) };
    let len = env.get_array_length(&array).expect("get_array_length");
    let mut strings = Vec::with_capacity(len as usize);
    for i in 0..len {
        let element = env.get_object_array_element(&array, i).expect("get_object_array_element");
        let jstr = jni::objects::JString::from(element);
        let s: String = env.get_string(&jstr).expect("get_string").into();
        strings.push(s);
    }
    let refs: Vec<&str> = strings.iter().map(String::as_str).collect();
    unsafe { borrow_mut::<StrutStyle>(ptr) }.set_font_families(&refs);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_StrutStyleNative_nWeight(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<StrutStyle>(ptr) }.font_style().weight
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_StrutStyleNative_nWidth(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<StrutStyle>(ptr) }.font_style().width
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_StrutStyleNative_nSlant(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    i32::from(unsafe { borrow::<StrutStyle>(ptr) }.font_style().slant)
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_StrutStyleNative_nSetFontStyle(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, weight: jint, width: jint, slant: jint) {
    unsafe { borrow_mut::<StrutStyle>(ptr) }.set_font_style(FontStyle::new(weight, width, slant.into()));
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_StrutStyleNative_nFontSize(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<StrutStyle>(ptr) }.font_size()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_StrutStyleNative_nSetFontSize(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, size: jfloat) {
    unsafe { borrow_mut::<StrutStyle>(ptr) }.set_font_size(size);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_StrutStyleNative_nHeight(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<StrutStyle>(ptr) }.height()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_StrutStyleNative_nSetHeight(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, height: jfloat) {
    unsafe { borrow_mut::<StrutStyle>(ptr) }.set_height(height);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_StrutStyleNative_nLeading(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<StrutStyle>(ptr) }.leading()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_StrutStyleNative_nSetLeading(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, leading: jfloat) {
    unsafe { borrow_mut::<StrutStyle>(ptr) }.set_leading(leading);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_StrutStyleNative_nStrutEnabled(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<StrutStyle>(ptr) }.strut_enabled() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_StrutStyleNative_nSetStrutEnabled(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, enabled: jboolean) {
    unsafe { borrow_mut::<StrutStyle>(ptr) }.set_strut_enabled(enabled != 0);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_StrutStyleNative_nForceStrutHeight(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<StrutStyle>(ptr) }.force_strut_height() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_StrutStyleNative_nSetForceStrutHeight(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, force: jboolean) {
    unsafe { borrow_mut::<StrutStyle>(ptr) }.set_force_strut_height(force != 0);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_StrutStyleNative_nHeightOverride(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<StrutStyle>(ptr) }.height_override() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_StrutStyleNative_nSetHeightOverride(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, height_override: jboolean) {
    unsafe { borrow_mut::<StrutStyle>(ptr) }.set_height_override(height_override != 0);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_StrutStyleNative_nHalfLeading(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<StrutStyle>(ptr) }.half_leading() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_StrutStyleNative_nSetHalfLeading(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, half_leading: jboolean) {
    unsafe { borrow_mut::<StrutStyle>(ptr) }.set_half_leading(half_leading != 0);
}
