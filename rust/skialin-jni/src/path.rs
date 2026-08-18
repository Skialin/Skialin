use jni::sys::{jboolean, jfloat, jint, jlong};
use jni::JNIEnv;

use skialin_core::{Path, PathBuilder, PathDirection, PathOp, Point, Rect};

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};

fn direction_from_ordinal(ordinal: jint) -> PathDirection {
    if ordinal == 1 {
        PathDirection::CounterClockwise
    } else {
        PathDirection::Clockwise
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nMake(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    box_ptr(PathBuilder::new())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<PathBuilder>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nMoveTo(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, x: jfloat, y: jfloat) {
    unsafe { borrow_mut::<PathBuilder>(ptr).move_to(Point::new(x, y)) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nLineTo(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, x: jfloat, y: jfloat) {
    unsafe { borrow_mut::<PathBuilder>(ptr).line_to(Point::new(x, y)) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nQuadTo(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    x1: jfloat,
    y1: jfloat,
    x2: jfloat,
    y2: jfloat,
) {
    unsafe { borrow_mut::<PathBuilder>(ptr).quad_to(Point::new(x1, y1), Point::new(x2, y2)) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nCubicTo(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    x1: jfloat,
    y1: jfloat,
    x2: jfloat,
    y2: jfloat,
    x3: jfloat,
    y3: jfloat,
) {
    unsafe { borrow_mut::<PathBuilder>(ptr).cubic_to(Point::new(x1, y1), Point::new(x2, y2), Point::new(x3, y3)) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nClose(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { borrow_mut::<PathBuilder>(ptr).close() };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nAddRect(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    left: jfloat,
    top: jfloat,
    right: jfloat,
    bottom: jfloat,
    direction: jint,
) {
    unsafe { borrow_mut::<PathBuilder>(ptr).add_rect(Rect::new(left, top, right, bottom), direction_from_ordinal(direction)) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nAddOval(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    left: jfloat,
    top: jfloat,
    right: jfloat,
    bottom: jfloat,
    direction: jint,
) {
    unsafe { borrow_mut::<PathBuilder>(ptr).add_oval(Rect::new(left, top, right, bottom), direction_from_ordinal(direction)) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nAddCircle(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    cx: jfloat,
    cy: jfloat,
    radius: jfloat,
    direction: jint,
) {
    unsafe { borrow_mut::<PathBuilder>(ptr).add_circle(Point::new(cx, cy), radius, direction_from_ordinal(direction)) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nOffset(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, dx: jfloat, dy: jfloat) {
    unsafe { borrow_mut::<PathBuilder>(ptr).offset(dx, dy) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nIsEmpty(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<PathBuilder>(ptr).is_empty() as jboolean }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nSnapshot(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    box_ptr(unsafe { borrow::<PathBuilder>(ptr) }.snapshot())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nDetach(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    box_ptr(unsafe { borrow_mut::<PathBuilder>(ptr) }.detach())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<Path>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathNative_nIsEmpty(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Path>(ptr).is_empty() as jboolean }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathNative_nGetBounds(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jni::sys::jfloatArray {
    let bounds = unsafe { borrow::<Path>(ptr) }.bounds();
    let out = [bounds.left, bounds.top, bounds.right, bounds.bottom];
    let array = env.new_float_array(4).expect("new_float_array");
    env.set_float_array_region(&array, 0, &out).expect("set_float_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathNative_nContains(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, x: jfloat, y: jfloat) -> jboolean {
    unsafe { borrow::<Path>(ptr) }.contains(Point::new(x, y)) as jboolean
}

fn path_op_from_ordinal(ordinal: jint) -> PathOp {
    match ordinal {
        1 => PathOp::Intersect,
        2 => PathOp::Union,
        3 => PathOp::Xor,
        4 => PathOp::ReverseDifference,
        _ => PathOp::Difference,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathNative_nOp(_env: JNIEnv, _class: jni::objects::JClass, one_ptr: jlong, two_ptr: jlong, op: jint) -> jlong {
    let one = unsafe { borrow::<Path>(one_ptr) };
    let two = unsafe { borrow::<Path>(two_ptr) };
    match Path::op(one, two, path_op_from_ordinal(op)) {
        Some(result) => box_ptr(result),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathNative_nSimplify(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    match unsafe { borrow::<Path>(ptr) }.simplify() {
        Some(result) => box_ptr(result),
        None => 0,
    }
}
