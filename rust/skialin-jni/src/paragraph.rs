use jni::sys::{jboolean, jfloat, jint, jlong};
use jni::JNIEnv;

use skialin_core::{Affinity, Paint, Paragraph, RectHeightStyle, RectWidthStyle, TextBox, TextDirection};

fn rect_height_style_from_ordinal(ordinal: jint) -> RectHeightStyle {
    match ordinal {
        1 => RectHeightStyle::Max,
        2 => RectHeightStyle::IncludeLineSpacingMiddle,
        3 => RectHeightStyle::IncludeLineSpacingTop,
        4 => RectHeightStyle::IncludeLineSpacingBottom,
        5 => RectHeightStyle::Strut,
        _ => RectHeightStyle::Tight,
    }
}

fn rect_width_style_from_ordinal(ordinal: jint) -> RectWidthStyle {
    if ordinal == 1 {
        RectWidthStyle::Max
    } else {
        RectWidthStyle::Tight
    }
}

use crate::util::{borrow, borrow_mut, drop_ptr};

fn flatten_boxes(boxes: Vec<TextBox>) -> Vec<f32> {
    let mut flat = Vec::with_capacity(boxes.len() * 5);
    for b in boxes {
        flat.push(b.rect.left);
        flat.push(b.rect.top);
        flat.push(b.rect.right);
        flat.push(b.rect.bottom);
        flat.push(if b.direction == TextDirection::Ltr { 1.0 } else { 0.0 });
    }
    flat
}

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

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nUnresolvedGlyphs(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow_mut::<Paragraph>(ptr) }.unresolved_glyphs().unwrap_or(-1)
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nGlyphPositionAtCoordinate(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, dx: jfloat, dy: jfloat) -> jlong {
    let pos = unsafe { borrow_mut::<Paragraph>(ptr) }.glyph_position_at_coordinate(dx, dy);
    let affinity = if pos.affinity == Affinity::Upstream { 0i64 } else { 1i64 };
    ((pos.position as i64) << 32) | affinity
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nWordBoundary<'l>(env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, ptr: jlong, offset: jint) -> jni::sys::jlongArray {
    let range = unsafe { borrow_mut::<Paragraph>(ptr) }.word_boundary(offset as u32);
    let array = env.new_long_array(2).expect("new_long_array");
    env.set_long_array_region(&array, 0, &[range.start as i64, range.end as i64]).expect("set_long_array_region");
    array.into_raw()
}

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

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nGetRectsForRange(
    env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    start: jint,
    end: jint,
    height_style: jint,
    width_style: jint,
) -> jni::sys::jfloatArray {
    let boxes = unsafe { borrow_mut::<Paragraph>(ptr) }.rects_for_range(
        start as u32,
        end as u32,
        rect_height_style_from_ordinal(height_style),
        rect_width_style_from_ordinal(width_style),
    );
    let flat = flatten_boxes(boxes);
    let array = env.new_float_array(flat.len() as i32).expect("new_float_array");
    env.set_float_array_region(&array, 0, &flat).expect("set_float_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nGetRectsForPlaceholders(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jni::sys::jfloatArray {
    let flat = flatten_boxes(unsafe { borrow_mut::<Paragraph>(ptr) }.rects_for_placeholders());
    let array = env.new_float_array(flat.len() as i32).expect("new_float_array");
    env.set_float_array_region(&array, 0, &flat).expect("set_float_array_region");
    array.into_raw()
}

fn write_glyph_info(env: &JNIEnv, out: jni::sys::jdoubleArray, info: Option<skialin_core::GlyphInfo>) -> jboolean {
    match info {
        Some(info) => {
            let values = [
                info.bounds.left as f64,
                info.bounds.top as f64,
                info.bounds.right as f64,
                info.bounds.bottom as f64,
                info.grapheme_cluster_range.start as f64,
                info.grapheme_cluster_range.end as f64,
                if info.direction == TextDirection::Ltr { 1.0 } else { 0.0 },
                if info.is_ellipsis { 1.0 } else { 0.0 },
            ];
            let array = unsafe { jni::objects::JDoubleArray::from_raw(out) };
            env.set_double_array_region(&array, 0, &values).expect("set_double_array_region");
            true as jboolean
        }
        None => false as jboolean,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nGlyphInfoAtUTF16Offset(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, code_unit_index: jlong, out: jni::sys::jdoubleArray) -> jboolean {
    let info = unsafe { borrow_mut::<Paragraph>(ptr) }.glyph_info_at_utf16_offset(code_unit_index as usize);
    write_glyph_info(&env, out, info)
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nClosestGlyphInfoAt(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, dx: jfloat, dy: jfloat, out: jni::sys::jdoubleArray) -> jboolean {
    let info = unsafe { borrow_mut::<Paragraph>(ptr) }.closest_glyph_info_at(dx, dy);
    write_glyph_info(&env, out, info)
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nUpdateFontSize(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, from: jlong, to: jlong, font_size: jfloat) {
    unsafe { borrow_mut::<Paragraph>(ptr) }.update_font_size(from as usize, to as usize, font_size);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nUpdateForegroundPaint(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, from: jlong, to: jlong, paint_ptr: jlong) {
    let paint = unsafe { borrow::<Paint>(paint_ptr) };
    unsafe { borrow_mut::<Paragraph>(ptr) }.update_foreground_paint(from as usize, to as usize, paint);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ParagraphNative_nUpdateBackgroundPaint(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, from: jlong, to: jlong, paint_ptr: jlong) {
    let paint = unsafe { borrow::<Paint>(paint_ptr) };
    unsafe { borrow_mut::<Paragraph>(ptr) }.update_background_paint(from as usize, to as usize, paint);
}
