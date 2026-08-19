use jni::sys::{jfloat, jfloatArray, jint, jlong};
use jni::JNIEnv;

use skialin_core::{Data, Font, Paint, Point, TextBlob, TextEncoding};

use crate::util::{borrow, box_ptr, drop_ptr};

fn encoding_from_ordinal(ordinal: jint) -> TextEncoding {
    match ordinal {
        1 => TextEncoding::Utf16,
        2 => TextEncoding::Utf32,
        3 => TextEncoding::GlyphId,
        _ => TextEncoding::Utf8,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextBlobNative_nFromText<'l>(
    mut env: JNIEnv<'l>,
    _class: jni::objects::JClass<'l>,
    text: jni::objects::JString<'l>,
    font_ptr: jlong,
    encoding: jint,
) -> jlong {
    let text: String = env.get_string(&text).expect("get_string").into();
    let font = unsafe { borrow::<Font>(font_ptr) };
    match TextBlob::from_text(&text, font, encoding_from_ordinal(encoding)) {
        Some(blob) => box_ptr(blob),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextBlobNative_nFromPosTextH<'l>(
    mut env: JNIEnv<'l>,
    _class: jni::objects::JClass<'l>,
    text: jni::objects::JString<'l>,
    xpos: jfloatArray,
    const_y: jfloat,
    font_ptr: jlong,
    encoding: jint,
) -> jlong {
    let text: String = env.get_string(&text).expect("get_string").into();
    let xpos_array = unsafe { jni::objects::JFloatArray::from_raw(xpos) };
    let len = env.get_array_length(&xpos_array).expect("get_array_length") as usize;
    let mut xpos_buf = vec![0f32; len];
    env.get_float_array_region(&xpos_array, 0, &mut xpos_buf).expect("get_float_array_region");

    let font = unsafe { borrow::<Font>(font_ptr) };
    match TextBlob::from_pos_text_h(&text, &xpos_buf, const_y, font, encoding_from_ordinal(encoding)) {
        Some(blob) => box_ptr(blob),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextBlobNative_nFromPosText<'l>(
    mut env: JNIEnv<'l>,
    _class: jni::objects::JClass<'l>,
    text: jni::objects::JString<'l>,
    pos: jfloatArray,
    font_ptr: jlong,
    encoding: jint,
) -> jlong {
    let text: String = env.get_string(&text).expect("get_string").into();
    let pos_array = unsafe { jni::objects::JFloatArray::from_raw(pos) };
    let len = env.get_array_length(&pos_array).expect("get_array_length") as usize;
    let mut pos_buf = vec![0f32; len];
    env.get_float_array_region(&pos_array, 0, &mut pos_buf).expect("get_float_array_region");
    let points: Vec<Point> = pos_buf.chunks_exact(2).map(|c| Point::new(c[0], c[1])).collect();

    let font = unsafe { borrow::<Font>(font_ptr) };
    match TextBlob::from_pos_text(&text, &points, font, encoding_from_ordinal(encoding)) {
        Some(blob) => box_ptr(blob),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextBlobNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<TextBlob>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextBlobNative_nUniqueId(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<TextBlob>(ptr) }.unique_id() as jint
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextBlobNative_nBounds(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, out: jfloatArray) {
    let bounds = unsafe { borrow::<TextBlob>(ptr) }.bounds();
    let values = [bounds.left, bounds.top, bounds.right, bounds.bottom];
    let array = unsafe { jni::objects::JFloatArray::from_raw(out) };
    env.set_float_array_region(&array, 0, &values).expect("set_float_array_region");
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextBlobNative_nFromRSXform<'l>(
    mut env: JNIEnv<'l>,
    _class: jni::objects::JClass<'l>,
    text: jni::objects::JString<'l>,
    xforms: jfloatArray,
    font_ptr: jlong,
    encoding: jint,
) -> jlong {
    let text: String = env.get_string(&text).expect("get_string").into();
    let array = unsafe { jni::objects::JFloatArray::from_raw(xforms) };
    let len = env.get_array_length(&array).expect("get_array_length") as usize;
    let mut buf = vec![0f32; len];
    env.get_float_array_region(&array, 0, &mut buf).expect("get_float_array_region");
    let font = unsafe { borrow::<Font>(font_ptr) };
    match TextBlob::from_rsxform(&text, &buf, font, encoding_from_ordinal(encoding)) {
        Some(blob) => box_ptr(blob),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextBlobNative_nGetIntercepts(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, lower: jfloat, upper: jfloat, paint_ptr: jlong) -> jfloatArray {
    let paint = (paint_ptr != 0).then(|| unsafe { borrow::<Paint>(paint_ptr) });
    let intervals = unsafe { borrow::<TextBlob>(ptr) }.get_intercepts(lower, upper, paint);
    let array = env.new_float_array(intervals.len() as i32).expect("new_float_array");
    env.set_float_array_region(&array, 0, &intervals).expect("set_float_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextBlobNative_nSerializeToData(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    box_ptr(unsafe { borrow::<TextBlob>(ptr) }.serialize_to_data())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextBlobNative_nFromData(_env: JNIEnv, _class: jni::objects::JClass, data_ptr: jlong) -> jlong {
    let data = unsafe { borrow::<Data>(data_ptr) };
    match TextBlob::from_data(data) {
        Some(blob) => box_ptr(blob),
        None => 0,
    }
}
