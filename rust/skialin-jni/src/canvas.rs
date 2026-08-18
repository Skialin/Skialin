use jni::sys::{jfloat, jint, jlong};
use jni::JNIEnv;

use skialin_core::{Canvas, ClipOp, Paint, Path, Point, Rect, TextBlob};

use crate::paint::blend_mode_from_ordinal;
use crate::util::borrow;

fn canvas_from_ptr<'a>(ptr: jlong) -> Canvas<'a> {
    unsafe { Canvas::from_raw(ptr as *mut skialin_core::sys::SkCanvas) }
}

fn clip_op_from_ordinal(ordinal: jint) -> ClipOp {
    if ordinal == 0 {
        ClipOp::Difference
    } else {
        ClipOp::Intersect
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nClear(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, color: jint) {
    canvas_from_ptr(ptr).clear(color as u32);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawColor(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, color: jint, mode: jint) {
    canvas_from_ptr(ptr).draw_color(color as u32, blend_mode_from_ordinal(mode));
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawPaint(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, paint_ptr: jlong) {
    canvas_from_ptr(ptr).draw_paint(unsafe { borrow::<Paint>(paint_ptr) });
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawLine(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    x0: jfloat,
    y0: jfloat,
    x1: jfloat,
    y1: jfloat,
    paint_ptr: jlong,
) {
    canvas_from_ptr(ptr).draw_line(Point::new(x0, y0), Point::new(x1, y1), unsafe { borrow::<Paint>(paint_ptr) });
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawRect(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    left: jfloat,
    top: jfloat,
    right: jfloat,
    bottom: jfloat,
    paint_ptr: jlong,
) {
    canvas_from_ptr(ptr).draw_rect(Rect::new(left, top, right, bottom), unsafe { borrow::<Paint>(paint_ptr) });
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawOval(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    left: jfloat,
    top: jfloat,
    right: jfloat,
    bottom: jfloat,
    paint_ptr: jlong,
) {
    canvas_from_ptr(ptr).draw_oval(Rect::new(left, top, right, bottom), unsafe { borrow::<Paint>(paint_ptr) });
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawCircle(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    cx: jfloat,
    cy: jfloat,
    radius: jfloat,
    paint_ptr: jlong,
) {
    canvas_from_ptr(ptr).draw_circle(Point::new(cx, cy), radius, unsafe { borrow::<Paint>(paint_ptr) });
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawPath(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, path_ptr: jlong, paint_ptr: jlong) {
    canvas_from_ptr(ptr).draw_path(unsafe { borrow::<Path>(path_ptr) }, unsafe { borrow::<Paint>(paint_ptr) });
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawTextBlob(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    blob_ptr: jlong,
    x: jfloat,
    y: jfloat,
    paint_ptr: jlong,
) {
    canvas_from_ptr(ptr).draw_text_blob(unsafe { borrow::<TextBlob>(blob_ptr) }, x, y, unsafe { borrow::<Paint>(paint_ptr) });
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nSave(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    canvas_from_ptr(ptr).save()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nRestore(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    canvas_from_ptr(ptr).restore();
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nRestoreToCount(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, save_count: jint) {
    canvas_from_ptr(ptr).restore_to_count(save_count);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nTranslate(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, dx: jfloat, dy: jfloat) {
    canvas_from_ptr(ptr).translate(dx, dy);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nScale(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, sx: jfloat, sy: jfloat) {
    canvas_from_ptr(ptr).scale(sx, sy);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nRotate(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, degrees: jfloat) {
    canvas_from_ptr(ptr).rotate(degrees);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nClipRect(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    left: jfloat,
    top: jfloat,
    right: jfloat,
    bottom: jfloat,
    op: jint,
) {
    canvas_from_ptr(ptr).clip_rect(Rect::new(left, top, right, bottom), clip_op_from_ordinal(op));
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nClipPath(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, path_ptr: jlong, op: jint) {
    canvas_from_ptr(ptr).clip_path(unsafe { borrow::<Path>(path_ptr) }, clip_op_from_ordinal(op));
}
