use jni::sys::{jfloat, jfloatArray, jlong, jshortArray};
use jni::JNIEnv;

use skialin_core::{Font, Point, TextBlobBuilder};

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};

fn read_shorts(env: &JNIEnv, glyphs: jshortArray) -> Vec<u16> {
    let array = unsafe { jni::objects::JShortArray::from_raw(glyphs) };
    let len = env.get_array_length(&array).expect("get_array_length") as usize;
    let mut buf = vec![0i16; len];
    env.get_short_array_region(&array, 0, &mut buf).expect("get_short_array_region");
    buf.into_iter().map(|g| g as u16).collect()
}

fn read_floats(env: &JNIEnv, values: jfloatArray) -> Vec<f32> {
    let array = unsafe { jni::objects::JFloatArray::from_raw(values) };
    let len = env.get_array_length(&array).expect("get_array_length") as usize;
    let mut buf = vec![0f32; len];
    env.get_float_array_region(&array, 0, &mut buf).expect("get_float_array_region");
    buf
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextBlobBuilderNative_nNew(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    box_ptr(TextBlobBuilder::new())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextBlobBuilderNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<TextBlobBuilder>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextBlobBuilderNative_nBuild(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    match unsafe { borrow_mut::<TextBlobBuilder>(ptr) }.build() {
        Some(blob) => box_ptr(blob),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextBlobBuilderNative_nAppendRun(
    env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    font_ptr: jlong,
    glyphs: jshortArray,
    x: jfloat,
    y: jfloat,
) {
    let glyphs = read_shorts(&env, glyphs);
    let font = unsafe { borrow::<Font>(font_ptr) };
    unsafe { borrow_mut::<TextBlobBuilder>(ptr) }.append_run(font, &glyphs, x, y);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextBlobBuilderNative_nAppendRunPosH(
    env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    font_ptr: jlong,
    glyphs: jshortArray,
    xpos: jfloatArray,
    y: jfloat,
) {
    let glyphs = read_shorts(&env, glyphs);
    let xpos = read_floats(&env, xpos);
    let font = unsafe { borrow::<Font>(font_ptr) };
    unsafe { borrow_mut::<TextBlobBuilder>(ptr) }.append_run_pos_h(font, &glyphs, &xpos, y);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextBlobBuilderNative_nAppendRunPos(
    env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    font_ptr: jlong,
    glyphs: jshortArray,
    pos: jfloatArray,
) {
    let glyphs = read_shorts(&env, glyphs);
    let pos_flat = read_floats(&env, pos);
    let points: Vec<Point> = pos_flat.chunks_exact(2).map(|c| Point::new(c[0], c[1])).collect();
    let font = unsafe { borrow::<Font>(font_ptr) };
    unsafe { borrow_mut::<TextBlobBuilder>(ptr) }.append_run_pos(font, &glyphs, &points);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_TextBlobBuilderNative_nAppendRunRSXform(
    env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    font_ptr: jlong,
    glyphs: jshortArray,
    xforms: jfloatArray,
) {
    let glyphs = read_shorts(&env, glyphs);
    let xforms = read_floats(&env, xforms);
    let font = unsafe { borrow::<Font>(font_ptr) };
    unsafe { borrow_mut::<TextBlobBuilder>(ptr) }.append_run_rsxform(font, &glyphs, &xforms);
}
