use jni::sys::{jboolean, jfloat, jint, jlong, JNI_TRUE};
use jni::JNIEnv;

use skialin_core::{BlendMode, ColorFilter, ImageFilter, MaskFilter, Paint, PaintStyle, Shader, StrokeCap, StrokeJoin};

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};

fn style_from_ordinal(ordinal: jint) -> PaintStyle {
    match ordinal {
        1 => PaintStyle::Stroke,
        2 => PaintStyle::StrokeAndFill,
        _ => PaintStyle::Fill,
    }
}

fn style_to_ordinal(style: PaintStyle) -> jint {
    match style {
        PaintStyle::Fill => 0,
        PaintStyle::Stroke => 1,
        PaintStyle::StrokeAndFill => 2,
    }
}

fn cap_from_ordinal(ordinal: jint) -> StrokeCap {
    match ordinal {
        1 => StrokeCap::Round,
        2 => StrokeCap::Square,
        _ => StrokeCap::Butt,
    }
}

fn cap_to_ordinal(cap: StrokeCap) -> jint {
    match cap {
        StrokeCap::Butt => 0,
        StrokeCap::Round => 1,
        StrokeCap::Square => 2,
    }
}

fn join_from_ordinal(ordinal: jint) -> StrokeJoin {
    match ordinal {
        1 => StrokeJoin::Round,
        2 => StrokeJoin::Bevel,
        _ => StrokeJoin::Miter,
    }
}

fn join_to_ordinal(join: StrokeJoin) -> jint {
    match join {
        StrokeJoin::Miter => 0,
        StrokeJoin::Round => 1,
        StrokeJoin::Bevel => 2,
    }
}

pub(crate) fn blend_mode_from_ordinal(ordinal: jint) -> BlendMode {
    use BlendMode::*;
    const MODES: [BlendMode; 15] = [
        Clear, Src, Dst, SrcOver, DstOver, SrcIn, DstIn, SrcOut, DstOut, SrcAtop, DstAtop, Xor, Plus, Modulate, Screen,
    ];
    MODES.get(ordinal as usize).copied().unwrap_or(SrcOver)
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PaintNative_nMake(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    box_ptr(Paint::new())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PaintNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<Paint>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PaintNative_nGetColor(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Paint>(ptr).color() as jint }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PaintNative_nSetColor(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, color: jint) {
    unsafe { borrow_mut::<Paint>(ptr).set_color(color as u32) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PaintNative_nIsAntiAlias(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Paint>(ptr).is_anti_alias() as jboolean }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PaintNative_nSetAntiAlias(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, anti_alias: jboolean) {
    unsafe { borrow_mut::<Paint>(ptr).set_anti_alias(anti_alias == JNI_TRUE) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PaintNative_nGetStyle(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    style_to_ordinal(unsafe { borrow::<Paint>(ptr).style() })
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PaintNative_nSetStyle(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, style: jint) {
    unsafe { borrow_mut::<Paint>(ptr).set_style(style_from_ordinal(style)) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PaintNative_nGetStrokeWidth(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<Paint>(ptr).stroke_width() }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PaintNative_nSetStrokeWidth(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, width: jfloat) {
    unsafe { borrow_mut::<Paint>(ptr).set_stroke_width(width) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PaintNative_nGetStrokeCap(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    cap_to_ordinal(unsafe { borrow::<Paint>(ptr).stroke_cap() })
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PaintNative_nSetStrokeCap(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, cap: jint) {
    unsafe { borrow_mut::<Paint>(ptr).set_stroke_cap(cap_from_ordinal(cap)) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PaintNative_nGetStrokeJoin(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    join_to_ordinal(unsafe { borrow::<Paint>(ptr).stroke_join() })
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PaintNative_nSetStrokeJoin(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, join: jint) {
    unsafe { borrow_mut::<Paint>(ptr).set_stroke_join(join_from_ordinal(join)) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PaintNative_nSetBlendMode(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, mode: jint) {
    unsafe { borrow_mut::<Paint>(ptr).set_blend_mode(blend_mode_from_ordinal(mode)) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PaintNative_nSetShader(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, shader_ptr: jlong) {
    let shader = (shader_ptr != 0).then(|| unsafe { borrow::<Shader>(shader_ptr) });
    unsafe { borrow_mut::<Paint>(ptr).set_shader(shader) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PaintNative_nSetColorFilter(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, filter_ptr: jlong) {
    let filter = (filter_ptr != 0).then(|| unsafe { borrow::<ColorFilter>(filter_ptr) });
    unsafe { borrow_mut::<Paint>(ptr).set_color_filter(filter) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PaintNative_nSetImageFilter(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, filter_ptr: jlong) {
    let filter = (filter_ptr != 0).then(|| unsafe { borrow::<ImageFilter>(filter_ptr) });
    unsafe { borrow_mut::<Paint>(ptr).set_image_filter(filter) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PaintNative_nSetMaskFilter(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, filter_ptr: jlong) {
    let filter = (filter_ptr != 0).then(|| unsafe { borrow::<MaskFilter>(filter_ptr) });
    unsafe { borrow_mut::<Paint>(ptr).set_mask_filter(filter) };
}
