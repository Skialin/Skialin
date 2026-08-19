use jni::sys::{jboolean, jfloat, jfloatArray, jint, jintArray, jlong};
use jni::JNIEnv;

use skialin_core::{Bitmap, Canvas, ClipOp, Color, Data, FilterMode, Font, Image, ImageFilter, IRect, Matrix, Paint, Path, Picture, Point, PointMode, RRect, Rect, Region, SamplingOptions, SrcRectConstraint, TextBlob, Vertices, M44};

use crate::paint::blend_mode_from_ordinal;
use crate::util::{borrow, borrow_mut, box_ptr};

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

fn point_mode_from_ordinal(ordinal: jint) -> PointMode {
    match ordinal {
        1 => PointMode::Lines,
        2 => PointMode::Polygon,
        _ => PointMode::Points,
    }
}

fn src_rect_constraint_from_ordinal(ordinal: jint) -> SrcRectConstraint {
    if ordinal == 1 {
        SrcRectConstraint::Fast
    } else {
        SrcRectConstraint::Strict
    }
}

fn sampling_from_args(max_aniso: jint, use_cubic: jboolean, cubic_b: f32, cubic_c: f32, filter: jint, mipmap: jint) -> SamplingOptions {
    SamplingOptions {
        max_aniso,
        cubic: (use_cubic != 0).then_some((cubic_b, cubic_c)),
        filter: if filter == 1 { skialin_core::FilterMode::Linear } else { skialin_core::FilterMode::Nearest },
        mipmap: match mipmap {
            1 => skialin_core::MipmapMode::Nearest,
            2 => skialin_core::MipmapMode::Linear,
            _ => skialin_core::MipmapMode::None,
        },
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

#[allow(clippy::too_many_arguments)]
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
    antialias: jboolean,
) {
    canvas_from_ptr(ptr).clip_rect(Rect::new(left, top, right, bottom), clip_op_from_ordinal(op), antialias != 0);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nClipPath(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, path_ptr: jlong, op: jint, antialias: jboolean) {
    canvas_from_ptr(ptr).clip_path(unsafe { borrow::<Path>(path_ptr) }, clip_op_from_ordinal(op), antialias != 0);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nSkew(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, sx: jfloat, sy: jfloat) {
    canvas_from_ptr(ptr).skew(sx, sy);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nResetMatrix(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    canvas_from_ptr(ptr).reset_matrix();
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nSetMatrix(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, matrix: jfloatArray) {
    let array = unsafe { jni::objects::JFloatArray::from_raw(matrix) };
    let mut values = [0f32; 9];
    env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
    canvas_from_ptr(ptr).set_matrix(&Matrix::from_array(values));
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nTotalMatrix(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloatArray {
    let matrix = canvas_from_ptr(ptr).total_matrix();
    let values = matrix.to_array();
    let array = env.new_float_array(9).expect("new_float_array");
    env.set_float_array_region(&array, 0, &values).expect("set_float_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nQuickRejectRect(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, left: jfloat, top: jfloat, right: jfloat, bottom: jfloat) -> jboolean {
    canvas_from_ptr(ptr).quick_reject_rect(Rect::new(left, top, right, bottom)) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nQuickRejectPath(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, path_ptr: jlong) -> jboolean {
    canvas_from_ptr(ptr).quick_reject_path(unsafe { borrow::<Path>(path_ptr) }) as jboolean
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawRoundRect(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    left: jfloat,
    top: jfloat,
    right: jfloat,
    bottom: jfloat,
    rx: jfloat,
    ry: jfloat,
    paint_ptr: jlong,
) {
    canvas_from_ptr(ptr).draw_round_rect(Rect::new(left, top, right, bottom), rx, ry, unsafe { borrow::<Paint>(paint_ptr) });
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawArc(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    left: jfloat,
    top: jfloat,
    right: jfloat,
    bottom: jfloat,
    start_angle: jfloat,
    sweep_angle: jfloat,
    use_center: jboolean,
    paint_ptr: jlong,
) {
    canvas_from_ptr(ptr).draw_arc(Rect::new(left, top, right, bottom), start_angle, sweep_angle, use_center != 0, unsafe { borrow::<Paint>(paint_ptr) });
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawPoints(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, mode: jint, points: jfloatArray, paint_ptr: jlong) {
    let array = unsafe { jni::objects::JFloatArray::from_raw(points) };
    let len = env.get_array_length(&array).expect("get_array_length") as usize;
    let mut buf = vec![0f32; len];
    env.get_float_array_region(&array, 0, &mut buf).expect("get_float_array_region");
    let pts: Vec<Point> = buf.chunks_exact(2).map(|c| Point::new(c[0], c[1])).collect();
    canvas_from_ptr(ptr).draw_points(point_mode_from_ordinal(mode), &pts, unsafe { borrow::<Paint>(paint_ptr) });
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawImage(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    image_ptr: jlong,
    x: jfloat,
    y: jfloat,
    max_aniso: jint,
    use_cubic: jboolean,
    cubic_b: jfloat,
    cubic_c: jfloat,
    filter: jint,
    mipmap: jint,
    paint_ptr: jlong,
) {
    let sampling = sampling_from_args(max_aniso, use_cubic, cubic_b, cubic_c, filter, mipmap);
    let paint = (paint_ptr != 0).then(|| unsafe { borrow::<Paint>(paint_ptr) });
    canvas_from_ptr(ptr).draw_image(unsafe { borrow::<Image>(image_ptr) }, x, y, sampling, paint);
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawImageRect(
    env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    image_ptr: jlong,
    src: jfloatArray,
    dst_left: jfloat,
    dst_top: jfloat,
    dst_right: jfloat,
    dst_bottom: jfloat,
    max_aniso: jint,
    use_cubic: jboolean,
    cubic_b: jfloat,
    cubic_c: jfloat,
    filter: jint,
    mipmap: jint,
    paint_ptr: jlong,
    constraint: jint,
) {
    let src_rect = if src.is_null() {
        None
    } else {
        let array = unsafe { jni::objects::JFloatArray::from_raw(src) };
        let mut values = [0f32; 4];
        env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
        Some(Rect::new(values[0], values[1], values[2], values[3]))
    };
    let sampling = sampling_from_args(max_aniso, use_cubic, cubic_b, cubic_c, filter, mipmap);
    let paint = (paint_ptr != 0).then(|| unsafe { borrow::<Paint>(paint_ptr) });
    canvas_from_ptr(ptr).draw_image_rect(
        unsafe { borrow::<Image>(image_ptr) },
        src_rect,
        Rect::new(dst_left, dst_top, dst_right, dst_bottom),
        sampling,
        paint,
        src_rect_constraint_from_ordinal(constraint),
    );
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawRRect(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, rrect_ptr: jlong, paint_ptr: jlong) {
    canvas_from_ptr(ptr).draw_rrect(unsafe { borrow::<RRect>(rrect_ptr) }, unsafe { borrow::<Paint>(paint_ptr) });
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawDRRect(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, outer_ptr: jlong, inner_ptr: jlong, paint_ptr: jlong) {
    canvas_from_ptr(ptr).draw_drrect(unsafe { borrow::<RRect>(outer_ptr) }, unsafe { borrow::<RRect>(inner_ptr) }, unsafe { borrow::<Paint>(paint_ptr) });
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nClipRRect(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, rrect_ptr: jlong, op: jint, antialias: jboolean) {
    canvas_from_ptr(ptr).clip_rrect(unsafe { borrow::<RRect>(rrect_ptr) }, clip_op_from_ordinal(op), antialias != 0);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawRegion(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, region_ptr: jlong, paint_ptr: jlong) {
    canvas_from_ptr(ptr).draw_region(unsafe { borrow::<Region>(region_ptr) }, unsafe { borrow::<Paint>(paint_ptr) });
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nClipRegion(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, region_ptr: jlong, op: jint) {
    canvas_from_ptr(ptr).clip_region(unsafe { borrow::<Region>(region_ptr) }, clip_op_from_ordinal(op));
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawVertices(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, vertices_ptr: jlong, mode: jint, paint_ptr: jlong) {
    canvas_from_ptr(ptr).draw_vertices(unsafe { borrow::<Vertices>(vertices_ptr) }, blend_mode_from_ordinal(mode), unsafe { borrow::<Paint>(paint_ptr) });
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nConcat44(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, matrix_ptr: jlong) {
    canvas_from_ptr(ptr).concat_44(unsafe { borrow::<M44>(matrix_ptr) });
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nSaveLayer(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, bounds: jfloatArray, paint_ptr: jlong) -> jint {
    let bounds_rect = if bounds.is_null() {
        None
    } else {
        let array = unsafe { jni::objects::JFloatArray::from_raw(bounds) };
        let mut values = [0f32; 4];
        env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
        Some(Rect::new(values[0], values[1], values[2], values[3]))
    };
    let paint = (paint_ptr != 0).then(|| unsafe { borrow::<Paint>(paint_ptr) });
    canvas_from_ptr(ptr).save_layer(bounds_rect, paint)
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nSaveLayerWithBackdrop(
    env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    bounds: jfloatArray,
    paint_ptr: jlong,
    backdrop_ptr: jlong,
    flags: jint,
) -> jint {
    let bounds_rect = if bounds.is_null() {
        None
    } else {
        let array = unsafe { jni::objects::JFloatArray::from_raw(bounds) };
        let mut values = [0f32; 4];
        env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
        Some(Rect::new(values[0], values[1], values[2], values[3]))
    };
    let paint = (paint_ptr != 0).then(|| unsafe { borrow::<Paint>(paint_ptr) });
    let backdrop = (backdrop_ptr != 0).then(|| unsafe { borrow::<ImageFilter>(backdrop_ptr) });
    canvas_from_ptr(ptr).save_layer_with_backdrop(bounds_rect, paint, backdrop, flags as u32)
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawImageNine(
    env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    image_ptr: jlong,
    center: jintArray,
    dst_left: jfloat,
    dst_top: jfloat,
    dst_right: jfloat,
    dst_bottom: jfloat,
    filter: jint,
    paint_ptr: jlong,
) {
    let array = unsafe { jni::objects::JIntArray::from_raw(center) };
    let mut values = [0i32; 4];
    env.get_int_array_region(&array, 0, &mut values).expect("get_int_array_region");
    let center_rect = IRect::new(values[0], values[1], values[2], values[3]);
    let filter_mode = if filter == 1 { FilterMode::Linear } else { FilterMode::Nearest };
    let paint = (paint_ptr != 0).then(|| unsafe { borrow::<Paint>(paint_ptr) });
    canvas_from_ptr(ptr).draw_image_nine(unsafe { borrow::<Image>(image_ptr) }, center_rect, Rect::new(dst_left, dst_top, dst_right, dst_bottom), filter_mode, paint);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawPatch(
    env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    cubics: jfloatArray,
    colors: jintArray,
    tex_coords: jfloatArray,
    mode: jint,
    paint_ptr: jlong,
) {
    let cubics_array = unsafe { jni::objects::JFloatArray::from_raw(cubics) };
    let mut cubics_flat = [0f32; 24];
    env.get_float_array_region(&cubics_array, 0, &mut cubics_flat).expect("get_float_array_region");
    let cubics_points: [Point; 12] = std::array::from_fn(|i| Point::new(cubics_flat[i * 2], cubics_flat[i * 2 + 1]));

    let colors_array = unsafe { jni::objects::JIntArray::from_raw(colors) };
    let mut colors_flat = [0i32; 4];
    env.get_int_array_region(&colors_array, 0, &mut colors_flat).expect("get_int_array_region");
    let colors_argb: [Color; 4] = std::array::from_fn(|i| colors_flat[i] as u32);

    let tex = if tex_coords.is_null() {
        None
    } else {
        let tex_array = unsafe { jni::objects::JFloatArray::from_raw(tex_coords) };
        let mut tex_flat = [0f32; 8];
        env.get_float_array_region(&tex_array, 0, &mut tex_flat).expect("get_float_array_region");
        Some(std::array::from_fn::<Point, 4, _>(|i| Point::new(tex_flat[i * 2], tex_flat[i * 2 + 1])))
    };

    let paint = unsafe { borrow::<Paint>(paint_ptr) };
    canvas_from_ptr(ptr).draw_patch(cubics_points, colors_argb, tex, blend_mode_from_ordinal(mode), paint);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawString<'l>(
    mut env: JNIEnv<'l>,
    _class: jni::objects::JClass<'l>,
    ptr: jlong,
    text: jni::objects::JString<'l>,
    x: jfloat,
    y: jfloat,
    font_ptr: jlong,
    paint_ptr: jlong,
) {
    let text: String = env.get_string(&text).expect("get_string").into();
    let owned_font;
    let font: &Font = if font_ptr != 0 {
        unsafe { borrow::<Font>(font_ptr) }
    } else {
        owned_font = Font::new();
        &owned_font
    };
    canvas_from_ptr(ptr).draw_string(&text, x, y, font, unsafe { borrow::<Paint>(paint_ptr) });
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nGetSaveCount(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    canvas_from_ptr(ptr).save_count()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nRotatePivot(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, degrees: jfloat, px: jfloat, py: jfloat) {
    canvas_from_ptr(ptr).rotate_pivot(degrees, px, py);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nReadPixels(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, bitmap_ptr: jlong, src_x: jint, src_y: jint) -> jboolean {
    canvas_from_ptr(ptr).read_pixels_bitmap(unsafe { borrow_mut::<Bitmap>(bitmap_ptr) }, src_x, src_y) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nWritePixels(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, bitmap_ptr: jlong, x: jint, y: jint) -> jboolean {
    canvas_from_ptr(ptr).write_pixels_bitmap(unsafe { borrow::<Bitmap>(bitmap_ptr) }, x, y) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawPictureMatrix(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, picture_ptr: jlong, matrix: jfloatArray, paint_ptr: jlong) {
    let sk_matrix = if matrix.is_null() {
        None
    } else {
        let array = unsafe { jni::objects::JFloatArray::from_raw(matrix) };
        let mut values = [0f32; 9];
        env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
        Some(Matrix::from_array(values))
    };
    let paint = (paint_ptr != 0).then(|| unsafe { borrow::<Paint>(paint_ptr) });
    canvas_from_ptr(ptr).draw_picture_with_matrix(unsafe { borrow::<Picture>(picture_ptr) }, sk_matrix.as_ref(), paint);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawDrawableAt(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, drawable_ptr: jlong, x: jfloat, y: jfloat) {
    canvas_from_ptr(ptr).draw_drawable_at(unsafe { borrow::<skialin_core::Drawable>(drawable_ptr) }, x, y);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nConcat(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, matrix: jfloatArray) {
    let array = unsafe { jni::objects::JFloatArray::from_raw(matrix) };
    let mut values = [0f32; 9];
    env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
    canvas_from_ptr(ptr).concat(&Matrix::from_array(values));
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nLocalToDevice(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    box_ptr(canvas_from_ptr(ptr).local_to_device())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawAnnotation<'l>(
    mut env: JNIEnv<'l>,
    _class: jni::objects::JClass<'l>,
    ptr: jlong,
    left: jfloat,
    top: jfloat,
    right: jfloat,
    bottom: jfloat,
    key: jni::objects::JString<'l>,
    value_ptr: jlong,
) {
    let key: String = env.get_string(&key).expect("get_string").into();
    let value = (value_ptr != 0).then(|| unsafe { borrow::<Data>(value_ptr) });
    canvas_from_ptr(ptr).draw_annotation(Rect::new(left, top, right, bottom), &key, value);
}
