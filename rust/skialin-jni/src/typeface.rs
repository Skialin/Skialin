use jni::sys::{jboolean, jbyteArray, jfloatArray, jint, jintArray, jlong, jstring};
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

#[no_mangle]
pub extern "system" fn Java_org_skialin_TypefaceNative_nTableTags(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jintArray {
    let tags = unsafe { borrow::<Typeface>(ptr) }.table_tags();
    let signed: Vec<i32> = tags.into_iter().map(|t| t as i32).collect();
    let array = env.new_int_array(signed.len() as i32).expect("new_int_array");
    env.set_int_array_region(&array, 0, &signed).expect("set_int_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TypefaceNative_nTableSize(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, tag: jint) -> jlong {
    unsafe { borrow::<Typeface>(ptr) }.table_size(tag as u32) as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TypefaceNative_nTableData(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, tag: jint, offset: jlong, length: jlong) -> jbyteArray {
    let data = unsafe { borrow::<Typeface>(ptr) }.table_data(tag as u32, offset as usize, length as usize);
    let signed: Vec<i8> = data.into_iter().map(|b| b as i8).collect();
    let array = env.new_byte_array(signed.len() as i32).expect("new_byte_array");
    env.set_byte_array_region(&array, 0, &signed).expect("set_byte_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TypefaceNative_nMakeClone(
    env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    axis_tags: jintArray,
    axis_values: jfloatArray,
    collection_index: jint,
) -> jlong {
    let axis_tags = unsafe { jni::objects::JIntArray::from_raw(axis_tags) };
    let axis_values = unsafe { jni::objects::JFloatArray::from_raw(axis_values) };
    let len = env.get_array_length(&axis_tags).expect("get_array_length") as usize;
    let mut tags = vec![0i32; len];
    env.get_int_array_region(&axis_tags, 0, &mut tags).expect("get_int_array_region");
    let mut values = vec![0f32; len];
    env.get_float_array_region(&axis_values, 0, &mut values).expect("get_float_array_region");
    let axes: Vec<(u32, f32)> = tags.into_iter().map(|t| t as u32).zip(values).collect();
    match unsafe { borrow::<Typeface>(ptr) }.make_clone(&axes, collection_index) {
        Some(clone) => box_ptr(clone),
        None => 0,
    }
}
