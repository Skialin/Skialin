use jni::sys::{jboolean, jfloat, jfloatArray, jint, jlong, jshortArray};
use jni::JNIEnv;

use skialin_core::{Edging, Font, Hinting, Typeface};

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};

fn edging_from_ordinal(ordinal: jint) -> Edging {
    match ordinal {
        1 => Edging::AntiAlias,
        2 => Edging::SubpixelAntiAlias,
        _ => Edging::Alias,
    }
}

fn hinting_from_ordinal(ordinal: jint) -> Hinting {
    match ordinal {
        1 => Hinting::Slight,
        2 => Hinting::Normal,
        3 => Hinting::Full,
        _ => Hinting::None,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nMakeDefault(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    box_ptr(Font::new())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nMakeWithTypeface(_env: JNIEnv, _class: jni::objects::JClass, typeface_ptr: jlong, size: jfloat) -> jlong {
    let typeface = unsafe { borrow::<Typeface>(typeface_ptr) };
    box_ptr(Font::from_typeface(typeface, size))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<Font>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nTypeface(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    match unsafe { borrow::<Font>(ptr) }.typeface() {
        Some(typeface) => box_ptr(typeface),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nSetTypeface(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, typeface_ptr: jlong) {
    let typeface = (typeface_ptr != 0).then(|| unsafe { borrow::<Typeface>(typeface_ptr) });
    unsafe { borrow_mut::<Font>(ptr) }.set_typeface(typeface);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nSize(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<Font>(ptr) }.size()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nSetSize(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, size: jfloat) {
    unsafe { borrow_mut::<Font>(ptr) }.set_size(size);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nScaleX(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<Font>(ptr) }.scale_x()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nSetScaleX(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, scale_x: jfloat) {
    unsafe { borrow_mut::<Font>(ptr) }.set_scale_x(scale_x);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nSkewX(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<Font>(ptr) }.skew_x()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nSetSkewX(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, skew_x: jfloat) {
    unsafe { borrow_mut::<Font>(ptr) }.set_skew_x(skew_x);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nEdging(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Font>(ptr) }.edging() as jint
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nSetEdging(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, edging: jint) {
    unsafe { borrow_mut::<Font>(ptr) }.set_edging(edging_from_ordinal(edging));
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nHinting(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Font>(ptr) }.hinting() as jint
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nSetHinting(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, hinting: jint) {
    unsafe { borrow_mut::<Font>(ptr) }.set_hinting(hinting_from_ordinal(hinting));
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nIsSubpixel(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Font>(ptr) }.is_subpixel() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nSetSubpixel(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, subpixel: jboolean) {
    unsafe { borrow_mut::<Font>(ptr) }.set_subpixel(subpixel != 0);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nIsEmbolden(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Font>(ptr) }.is_embolden() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nSetEmbolden(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, embolden: jboolean) {
    unsafe { borrow_mut::<Font>(ptr) }.set_embolden(embolden != 0);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nIsLinearMetrics(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Font>(ptr) }.is_linear_metrics() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nSetLinearMetrics(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, linear_metrics: jboolean) {
    unsafe { borrow_mut::<Font>(ptr) }.set_linear_metrics(linear_metrics != 0);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nIsForceAutoHinting(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Font>(ptr) }.is_force_auto_hinting() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nSetForceAutoHinting(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, force_auto_hinting: jboolean) {
    unsafe { borrow_mut::<Font>(ptr) }.set_force_auto_hinting(force_auto_hinting != 0);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nIsEmbeddedBitmaps(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Font>(ptr) }.is_embedded_bitmaps() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nSetEmbeddedBitmaps(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, embedded_bitmaps: jboolean) {
    unsafe { borrow_mut::<Font>(ptr) }.set_embedded_bitmaps(embedded_bitmaps != 0);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nIsBaselineSnap(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Font>(ptr) }.is_baseline_snap() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nSetBaselineSnap(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, baseline_snap: jboolean) {
    unsafe { borrow_mut::<Font>(ptr) }.set_baseline_snap(baseline_snap != 0);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nUnicharToGlyph(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, unichar: jint) -> jint {
    unsafe { borrow::<Font>(ptr) }.unichar_to_glyph(unichar) as jint
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nTextToGlyphs<'l>(mut env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, ptr: jlong, text: jni::objects::JString<'l>) -> jshortArray {
    let text: String = env.get_string(&text).expect("get_string").into();
    let glyphs = unsafe { borrow::<Font>(ptr) }.text_to_glyphs(&text);
    let signed: Vec<i16> = glyphs.into_iter().map(|g| g as i16).collect();
    let array = env.new_short_array(signed.len() as i32).expect("new_short_array");
    env.set_short_array_region(&array, 0, &signed).expect("set_short_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nMeasureText<'l>(mut env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, ptr: jlong, text: jni::objects::JString<'l>) -> jfloat {
    let text: String = env.get_string(&text).expect("get_string").into();
    unsafe { borrow::<Font>(ptr) }.measure_text(&text)
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nWidths<'l>(env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, ptr: jlong, glyphs: jshortArray) -> jfloatArray {
    let glyphs = unsafe { jni::objects::JShortArray::from_raw(glyphs) };
    let len = env.get_array_length(&glyphs).expect("get_array_length") as usize;
    let mut buf = vec![0i16; len];
    env.get_short_array_region(&glyphs, 0, &mut buf).expect("get_short_array_region");
    let glyphs: Vec<u16> = buf.into_iter().map(|g| g as u16).collect();

    let widths = unsafe { borrow::<Font>(ptr) }.widths(&glyphs);
    let array = env.new_float_array(widths.len() as i32).expect("new_float_array");
    env.set_float_array_region(&array, 0, &widths).expect("set_float_array_region");
    array.into_raw()
}

/// Writes [top, ascent, descent, bottom, leading, avgCharWidth,
/// maxCharWidth, xMin, xMax, xHeight, capHeight] into `out`, an
/// 11-element float array allocated by the caller.
#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nMetrics(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, out: jfloatArray) {
    let metrics = unsafe { borrow::<Font>(ptr) }.metrics();
    let values = [
        metrics.top,
        metrics.ascent,
        metrics.descent,
        metrics.bottom,
        metrics.leading,
        metrics.avg_char_width,
        metrics.max_char_width,
        metrics.x_min,
        metrics.x_max,
        metrics.x_height,
        metrics.cap_height,
    ];
    let array = unsafe { jni::objects::JFloatArray::from_raw(out) };
    env.set_float_array_region(&array, 0, &values).expect("set_float_array_region");
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_FontNative_nSpacing(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<Font>(ptr) }.spacing()
}
