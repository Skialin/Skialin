use jni::sys::{jboolean, jfloat, jfloatArray, jint, jlong};
use jni::JNIEnv;

use skialin_core::{AddPathMode, Matrix, Path, PathBuilder, PathDirection, PathFillType, PathOp, PathVerb, Point, RRect, Rect};

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};

fn add_path_mode_from_ordinal(ordinal: jint) -> AddPathMode {
    if ordinal == 1 {
        AddPathMode::Extend
    } else {
        AddPathMode::Append
    }
}

fn direction_from_ordinal(ordinal: jint) -> PathDirection {
    if ordinal == 1 {
        PathDirection::CounterClockwise
    } else {
        PathDirection::Clockwise
    }
}

fn fill_type_from_ordinal(ordinal: jint) -> PathFillType {
    match ordinal {
        1 => PathFillType::EvenOdd,
        2 => PathFillType::InverseWinding,
        3 => PathFillType::InverseEvenOdd,
        _ => PathFillType::Winding,
    }
}

fn fill_type_to_ordinal(fill_type: PathFillType) -> jint {
    match fill_type {
        PathFillType::Winding => 0,
        PathFillType::EvenOdd => 1,
        PathFillType::InverseWinding => 2,
        PathFillType::InverseEvenOdd => 3,
    }
}

fn verb_to_ordinal(verb: PathVerb) -> jint {
    match verb {
        PathVerb::Move => 0,
        PathVerb::Line => 1,
        PathVerb::Quad => 2,
        PathVerb::Conic => 3,
        PathVerb::Cubic => 4,
        PathVerb::Close => 5,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nMake(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    box_ptr(PathBuilder::new())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nMakeFromPath(_env: JNIEnv, _class: jni::objects::JClass, path_ptr: jlong) -> jlong {
    let path = unsafe { borrow::<Path>(path_ptr) };
    box_ptr(PathBuilder::from_path(path))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<PathBuilder>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nSetFillType(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, fill_type: jint) {
    unsafe { borrow_mut::<PathBuilder>(ptr) }.set_fill_type(fill_type_from_ordinal(fill_type));
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nFillType(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    fill_type_to_ordinal(unsafe { borrow::<PathBuilder>(ptr) }.fill_type())
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nArcTo(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    left: jfloat,
    top: jfloat,
    right: jfloat,
    bottom: jfloat,
    start_angle_deg: jfloat,
    sweep_angle_deg: jfloat,
    force_move_to: jboolean,
) {
    unsafe { borrow_mut::<PathBuilder>(ptr) }.arc_to(Rect::new(left, top, right, bottom), start_angle_deg, sweep_angle_deg, force_move_to != 0);
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
pub extern "system" fn Java_org_skialin_PathBuilderNative_nRMoveTo(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, dx: jfloat, dy: jfloat) {
    unsafe { borrow_mut::<PathBuilder>(ptr).r_move_to(dx, dy) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nRLineTo(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, dx: jfloat, dy: jfloat) {
    unsafe { borrow_mut::<PathBuilder>(ptr).r_line_to(dx, dy) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nRQuadTo(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, dx1: jfloat, dy1: jfloat, dx2: jfloat, dy2: jfloat) {
    unsafe { borrow_mut::<PathBuilder>(ptr).r_quad_to(dx1, dy1, dx2, dy2) };
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nRCubicTo(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    dx1: jfloat,
    dy1: jfloat,
    dx2: jfloat,
    dy2: jfloat,
    dx3: jfloat,
    dy3: jfloat,
) {
    unsafe { borrow_mut::<PathBuilder>(ptr).r_cubic_to(dx1, dy1, dx2, dy2, dx3, dy3) };
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nConicTo(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    x1: jfloat,
    y1: jfloat,
    x2: jfloat,
    y2: jfloat,
    w: jfloat,
) {
    unsafe { borrow_mut::<PathBuilder>(ptr).conic_to(Point::new(x1, y1), Point::new(x2, y2), w) };
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nRConicTo(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    dx1: jfloat,
    dy1: jfloat,
    dx2: jfloat,
    dy2: jfloat,
    w: jfloat,
) {
    unsafe { borrow_mut::<PathBuilder>(ptr).r_conic_to(dx1, dy1, dx2, dy2, w) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nAddRRect(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, rrect_ptr: jlong, direction: jint) {
    let rrect = unsafe { borrow::<RRect>(rrect_ptr) };
    unsafe { borrow_mut::<PathBuilder>(ptr).add_rrect(rrect, direction_from_ordinal(direction)) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nAddPoly(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, points: jfloatArray, close: jboolean) {
    let array = unsafe { jni::objects::JFloatArray::from_raw(points) };
    let len = env.get_array_length(&array).expect("get_array_length") as usize;
    let mut buf = vec![0f32; len];
    env.get_float_array_region(&array, 0, &mut buf).expect("get_float_array_region");
    let pts: Vec<Point> = buf.chunks_exact(2).map(|c| Point::new(c[0], c[1])).collect();
    unsafe { borrow_mut::<PathBuilder>(ptr).add_poly(&pts, close != 0) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nAddPath(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    src_ptr: jlong,
    dx: jfloat,
    dy: jfloat,
    mode: jint,
) {
    let src = unsafe { borrow::<Path>(src_ptr) };
    unsafe { borrow_mut::<PathBuilder>(ptr).add_path(src, dx, dy, add_path_mode_from_ordinal(mode)) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nAddPathMatrix(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, src_ptr: jlong, matrix: jfloatArray, mode: jint) {
    let array = unsafe { jni::objects::JFloatArray::from_raw(matrix) };
    let mut values = [0f32; 9];
    env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
    let src = unsafe { borrow::<Path>(src_ptr) };
    unsafe { borrow_mut::<PathBuilder>(ptr).add_path_matrix(src, &Matrix::from_array(values), add_path_mode_from_ordinal(mode)) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nTransform(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, matrix: jfloatArray) {
    let array = unsafe { jni::objects::JFloatArray::from_raw(matrix) };
    let mut values = [0f32; 9];
    env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
    unsafe { borrow_mut::<PathBuilder>(ptr).transform(&Matrix::from_array(values)) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nSetLastPt(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, x: jfloat, y: jfloat) {
    unsafe { borrow_mut::<PathBuilder>(ptr).set_last_pt(x, y) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathBuilderNative_nReset(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { borrow_mut::<PathBuilder>(ptr).reset() };
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

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathNative_nFillType(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    match unsafe { borrow::<Path>(ptr) }.fill_type() {
        PathFillType::Winding => 0,
        PathFillType::EvenOdd => 1,
        PathFillType::InverseWinding => 2,
        PathFillType::InverseEvenOdd => 3,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathNative_nIsConvex(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Path>(ptr) }.is_convex() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathNative_nIsOval(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloatArray {
    match unsafe { borrow::<Path>(ptr) }.is_oval() {
        Some(bounds) => {
            let array = env.new_float_array(4).expect("new_float_array");
            let values = [bounds.left, bounds.top, bounds.right, bounds.bottom];
            env.set_float_array_region(&array, 0, &values).expect("set_float_array_region");
            array.into_raw()
        }
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathNative_nIsRRect(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    match unsafe { borrow::<Path>(ptr) }.is_rrect() {
        Some(rrect) => box_ptr(rrect),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathNative_nComputeTightBounds(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloatArray {
    let bounds = unsafe { borrow::<Path>(ptr) }.compute_tight_bounds();
    let array = env.new_float_array(4).expect("new_float_array");
    let values = [bounds.left, bounds.top, bounds.right, bounds.bottom];
    env.set_float_array_region(&array, 0, &values).expect("set_float_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathNative_nPointsCount(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Path>(ptr) }.points_count()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathNative_nPoints(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloatArray {
    let points = unsafe { borrow::<Path>(ptr) }.points();
    let mut flat = Vec::with_capacity(points.len() * 2);
    for p in points {
        flat.push(p.x);
        flat.push(p.y);
    }
    let array = env.new_float_array(flat.len() as i32).expect("new_float_array");
    env.set_float_array_region(&array, 0, &flat).expect("set_float_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PathNative_nGenerationId(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Path>(ptr) }.generation_id() as jint
}

/// Flattens this path's verbs into groups of 10 floats each:
/// `[verbOrdinal, x0,y0, x1,y1, x2,y2, x3,y3, conicWeight]`.
#[no_mangle]
pub extern "system" fn Java_org_skialin_PathNative_nSegments(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, convert_conics_to_quads: jboolean, tolerance: jfloat) -> jfloatArray {
    let segments = unsafe { borrow::<Path>(ptr) }.segments(convert_conics_to_quads != 0, tolerance);
    let mut flat = Vec::with_capacity(segments.len() * 10);
    for seg in segments {
        flat.push(verb_to_ordinal(seg.verb) as f32);
        for p in seg.points {
            flat.push(p.x);
            flat.push(p.y);
        }
        flat.push(seg.conic_weight);
    }
    let array = env.new_float_array(flat.len() as i32).expect("new_float_array");
    env.set_float_array_region(&array, 0, &flat).expect("set_float_array_region");
    array.into_raw()
}
