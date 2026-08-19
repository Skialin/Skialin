use jni::sys::{jboolean, jdouble, jfloat, jfloatArray, jint, jlong, jobjectArray, jstring};
use jni::JNIEnv;

use skialin_core::{FontStyle, Shadow, TextDecoration, TextDecorationMode, TextDecorationStyle, TextStyle, Typeface};

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nNew(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    box_ptr(TextStyle::new())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nClone(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    box_ptr(unsafe { borrow::<TextStyle>(ptr) }.clone_style())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<TextStyle>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nColor(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<TextStyle>(ptr) }.color() as jint
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nSetColor(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, color: jint) {
    unsafe { borrow_mut::<TextStyle>(ptr) }.set_color(color as u32);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nFontFamilies<'l>(mut env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, ptr: jlong) -> jobjectArray {
    let families = unsafe { borrow::<TextStyle>(ptr) }.font_families();
    let string_class = env.find_class("java/lang/String").expect("find_class");
    let array = env.new_object_array(families.len() as i32, string_class, unsafe { jni::objects::JObject::from_raw(std::ptr::null_mut()) }).expect("new_object_array");
    for (i, family) in families.iter().enumerate() {
        let jstr = env.new_string(family).expect("new_string");
        env.set_object_array_element(&array, i as i32, jstr).expect("set_object_array_element");
    }
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nSetFontFamilies<'l>(mut env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, ptr: jlong, families: jobjectArray) {
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
    unsafe { borrow_mut::<TextStyle>(ptr) }.set_font_families(&refs);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nFontSize(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<TextStyle>(ptr) }.font_size()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nSetFontSize(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, size: jfloat) {
    unsafe { borrow_mut::<TextStyle>(ptr) }.set_font_size(size);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nWeight(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<TextStyle>(ptr) }.font_style().weight
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nWidth(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<TextStyle>(ptr) }.font_style().width
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nSlant(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    i32::from(unsafe { borrow::<TextStyle>(ptr) }.font_style().slant)
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nSetFontStyle(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, weight: jint, width: jint, slant: jint) {
    unsafe { borrow_mut::<TextStyle>(ptr) }.set_font_style(FontStyle::new(weight, width, slant.into()));
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nDecorationType(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<TextStyle>(ptr) }.decoration().decoration.0
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nDecorationMode(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<TextStyle>(ptr) }.decoration().mode.into()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nDecorationColor(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<TextStyle>(ptr) }.decoration().color as jint
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nDecorationStyle(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<TextStyle>(ptr) }.decoration().style.into()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nDecorationThicknessMultiplier(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<TextStyle>(ptr) }.decoration().thickness_multiplier
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nSetDecoration(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, decoration: jint) {
    unsafe { borrow_mut::<TextStyle>(ptr) }.set_decoration(TextDecoration(decoration));
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nSetDecorationMode(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, mode: jint) {
    unsafe { borrow_mut::<TextStyle>(ptr) }.set_decoration_mode(TextDecorationMode::from(mode));
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nSetDecorationColor(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, color: jint) {
    unsafe { borrow_mut::<TextStyle>(ptr) }.set_decoration_color(color as u32);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nSetDecorationStyle(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, style: jint) {
    unsafe { borrow_mut::<TextStyle>(ptr) }.set_decoration_style(TextDecorationStyle::from(style));
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nSetDecorationThicknessMultiplier(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, multiplier: jfloat) {
    unsafe { borrow_mut::<TextStyle>(ptr) }.set_decoration_thickness_multiplier(multiplier);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nLetterSpacing(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<TextStyle>(ptr) }.letter_spacing()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nSetLetterSpacing(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, letter_spacing: jfloat) {
    unsafe { borrow_mut::<TextStyle>(ptr) }.set_letter_spacing(letter_spacing);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nWordSpacing(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<TextStyle>(ptr) }.word_spacing()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nSetWordSpacing(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, word_spacing: jfloat) {
    unsafe { borrow_mut::<TextStyle>(ptr) }.set_word_spacing(word_spacing);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nHeight(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<TextStyle>(ptr) }.height()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nSetHeight(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, height: jfloat) {
    unsafe { borrow_mut::<TextStyle>(ptr) }.set_height(height);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nHeightOverride(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<TextStyle>(ptr) }.height_override() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nSetHeightOverride(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, height_override: jboolean) {
    unsafe { borrow_mut::<TextStyle>(ptr) }.set_height_override(height_override != 0);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nShadows(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloatArray {
    let shadows = unsafe { borrow::<TextStyle>(ptr) }.shadows();
    let mut flat = Vec::with_capacity(shadows.len() * 4);
    for shadow in shadows {
        flat.push(f32::from_bits(shadow.color));
        flat.push(shadow.offset_x);
        flat.push(shadow.offset_y);
        flat.push(shadow.blur_sigma as f32);
    }
    let array = env.new_float_array(flat.len() as i32).expect("new_float_array");
    env.set_float_array_region(&array, 0, &flat).expect("set_float_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nAddShadow(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, color: jint, offset_x: jfloat, offset_y: jfloat, blur_sigma: jdouble) {
    unsafe { borrow_mut::<TextStyle>(ptr) }.add_shadow(Shadow { color: color as u32, offset_x, offset_y, blur_sigma });
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nResetShadows(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { borrow_mut::<TextStyle>(ptr) }.reset_shadows();
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nFontFeatureNames<'l>(mut env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, ptr: jlong) -> jobjectArray {
    let features = unsafe { borrow::<TextStyle>(ptr) }.font_features();
    let string_class = env.find_class("java/lang/String").expect("find_class");
    let array = env.new_object_array(features.len() as i32, string_class, unsafe { jni::objects::JObject::from_raw(std::ptr::null_mut()) }).expect("new_object_array");
    for (i, feature) in features.iter().enumerate() {
        let jstr = env.new_string(&feature.name).expect("new_string");
        env.set_object_array_element(&array, i as i32, jstr).expect("set_object_array_element");
    }
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nFontFeatureValues(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jni::sys::jintArray {
    let features = unsafe { borrow::<TextStyle>(ptr) }.font_features();
    let values: Vec<jint> = features.iter().map(|f| f.value).collect();
    let array = env.new_int_array(values.len() as i32).expect("new_int_array");
    env.set_int_array_region(&array, 0, &values).expect("set_int_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nAddFontFeature<'l>(mut env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, ptr: jlong, name: jni::objects::JString<'l>, value: jint) {
    let name: String = env.get_string(&name).expect("get_string").into();
    unsafe { borrow_mut::<TextStyle>(ptr) }.add_font_feature(&name, value);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nResetFontFeatures(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { borrow_mut::<TextStyle>(ptr) }.reset_font_features();
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nTypeface(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    match unsafe { borrow::<TextStyle>(ptr) }.typeface() {
        Some(typeface) => box_ptr(typeface),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nSetTypeface(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, typeface_ptr: jlong) {
    let typeface = (typeface_ptr != 0).then(|| unsafe { borrow::<Typeface>(typeface_ptr) });
    unsafe { borrow_mut::<TextStyle>(ptr) }.set_typeface(typeface);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nLocale<'l>(env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, ptr: jlong) -> jstring {
    let locale = unsafe { borrow::<TextStyle>(ptr) }.locale();
    env.new_string(locale).expect("new_string").into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextStyleNative_nSetLocale<'l>(mut env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, ptr: jlong, locale: jni::objects::JString<'l>) {
    let locale: String = env.get_string(&locale).expect("get_string").into();
    unsafe { borrow_mut::<TextStyle>(ptr) }.set_locale(&locale);
}
