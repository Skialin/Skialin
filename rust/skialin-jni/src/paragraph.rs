use jni::sys::{jboolean, jfloat, jint, jlong};
use jni::JNIEnv;

use skialin_core::{Affinity, Paragraph};

use crate::util::{borrow, borrow_mut, drop_ptr};

fn canvas_from_ptr<'a>(ptr: jlong) -> skialin_core::Canvas<'a> {
    unsafe { skialin_core::Canvas::from_raw(ptr as *mut skialin_core::sys::SkCanvas) }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<Paragraph>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nLayout(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, width: jfloat) {
    unsafe { borrow_mut::<Paragraph>(ptr) }.layout(width);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nPaint(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, canvas_ptr: jlong, x: jfloat, y: jfloat) {
    let mut canvas = canvas_from_ptr(canvas_ptr);
    unsafe { borrow_mut::<Paragraph>(ptr) }.paint(&mut canvas, x, y);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nMaxWidth(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<Paragraph>(ptr) }.max_width()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nHeight(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<Paragraph>(ptr) }.height()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nMinIntrinsicWidth(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<Paragraph>(ptr) }.min_intrinsic_width()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nMaxIntrinsicWidth(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<Paragraph>(ptr) }.max_intrinsic_width()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nAlphabeticBaseline(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<Paragraph>(ptr) }.alphabetic_baseline()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nIdeographicBaseline(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<Paragraph>(ptr) }.ideographic_baseline()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nLongestLine(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<Paragraph>(ptr) }.longest_line()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nDidExceedMaxLines(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Paragraph>(ptr) }.did_exceed_max_lines() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nLineNumber(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    unsafe { borrow_mut::<Paragraph>(ptr) }.line_number() as jlong
}

/// -1 if not applicable (not shaped yet).
#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nUnresolvedGlyphs(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow_mut::<Paragraph>(ptr) }.unresolved_glyphs().unwrap_or(-1)
}

/// Returns `[position, affinity]` where affinity is 0 (upstream) or 1 (downstream).
#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nGlyphPositionAtCoordinate(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, dx: jfloat, dy: jfloat) -> jlong {
    let pos = unsafe { borrow_mut::<Paragraph>(ptr) }.glyph_position_at_coordinate(dx, dy);
    let affinity = if pos.affinity == Affinity::Upstream { 0i64 } else { 1i64 };
    ((pos.position as i64) << 32) | affinity
}

/// Returns `[start, end)` of the word containing the glyph at `offset`, as `[start, end]`.
#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nWordBoundary<'l>(env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, ptr: jlong, offset: jint) -> jni::sys::jlongArray {
    let range = unsafe { borrow_mut::<Paragraph>(ptr) }.word_boundary(offset as u32);
    let array = env.new_long_array(2).expect("new_long_array");
    env.set_long_array_region(&array, 0, &[range.start as i64, range.end as i64]).expect("set_long_array_region");
    array.into_raw()
}

/// Writes [startIndex, endIndex, endExcludingWhitespaces, endIncludingNewline, hardBreak(0/1),
/// ascent, descent, unscaledAscent, height, width, left, baseline] into `out`, a 12-element
/// double array allocated by the caller. Returns false (leaving `out` unset) if `lineNumber`
/// is out of range.
#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nLineMetricsAt(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, line_number: jint, out: jni::sys::jdoubleArray) -> jboolean {
    match unsafe { borrow::<Paragraph>(ptr) }.line_metrics_at(line_number) {
        Some(m) => {
            let values = [
                m.start_index as f64,
                m.end_index as f64,
                m.end_excluding_whitespaces as f64,
                m.end_including_newline as f64,
                if m.hard_break { 1.0 } else { 0.0 },
                m.ascent,
                m.descent,
                m.unscaled_ascent,
                m.height,
                m.width,
                m.left,
                m.baseline,
            ];
            let array = unsafe { jni::objects::JDoubleArray::from_raw(out) };
            env.set_double_array_region(&array, 0, &values).expect("set_double_array_region");
            true as jboolean
        }
        None => false as jboolean,
    }
}
