use jni::sys::{jboolean, jfloat, jfloatArray, jint, jlong};
use jni::JNIEnv;

use skialin_core::canvas::ClipOp;
use skialin_core::{Canvas, Paint, Path, RRect, Rect, RenderNode, RenderNodeContext};

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};

fn clip_op_from_jint(v: jint) -> ClipOp {
    if v == 0 {
        ClipOp::Difference
    } else {
        ClipOp::Intersect
    }
}

fn rect_to_jfloatarray(env: &JNIEnv, rect: Rect) -> jfloatArray {
    let array = env.new_float_array(4).expect("new_float_array");
    env.set_float_array_region(&array, 0, &[rect.left, rect.top, rect.right, rect.bottom]).expect("set_float_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RenderNodeContextNative_nMake(_env: JNIEnv, _class: jni::objects::JClass, measure_draw_bounds: jboolean, snapshot_cache: jboolean) -> jlong {
    box_ptr(RenderNodeContext::new(measure_draw_bounds != 0, snapshot_cache != 0))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RenderNodeContextNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<RenderNodeContext>(ptr) };
}

#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub extern "system" fn Java_org_skialin_RenderNodeContextNative_nSetLightingInfo(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    center_x: jfloat,
    center_y: jfloat,
    center_z: jfloat,
    radius: jfloat,
    ambient_shadow_alpha: jfloat,
    spot_shadow_alpha: jfloat,
) {
    let context = unsafe { borrow_mut::<RenderNodeContext>(ptr) };
    context.set_lighting_info(center_x, center_y, center_z, radius, ambient_shadow_alpha, spot_shadow_alpha);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RenderNodeNative_nMake(_env: JNIEnv, _class: jni::objects::JClass, context_ptr: jlong) -> jlong {
    // RenderNode::new takes the context by reference: the C++ side takes its own sk_sp ref
    // internally (see skialin_bridge_RenderNode_Make), so there's no need for Rust-side shared
    // ownership (Rc) of the context the way a plain-Rust RenderNodeContext would have needed.
    let context = unsafe { borrow::<RenderNodeContext>(context_ptr) };
    box_ptr(RenderNode::new(context))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RenderNodeNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<RenderNode>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RenderNodeNative_nGetLayerPaint(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    match unsafe { borrow::<RenderNode>(ptr) }.layer_paint() {
        Some(paint) => box_ptr(paint),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RenderNodeNative_nSetLayerPaint(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, paint_ptr: jlong) {
    let paint = if paint_ptr == 0 { None } else { Some(unsafe { borrow::<Paint>(paint_ptr) }) };
    unsafe { borrow_mut::<RenderNode>(ptr) }.set_layer_paint(paint);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RenderNodeNative_nGetBounds(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloatArray {
    rect_to_jfloatarray(&env, unsafe { borrow::<RenderNode>(ptr) }.bounds())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RenderNodeNative_nSetBounds(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, left: jfloat, top: jfloat, right: jfloat, bottom: jfloat) {
    unsafe { borrow_mut::<RenderNode>(ptr) }.set_bounds(Rect::new(left, top, right, bottom));
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RenderNodeNative_nGetPivotX(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<RenderNode>(ptr) }.pivot().map_or(f32::NAN, |p| p.0)
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RenderNodeNative_nGetPivotY(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<RenderNode>(ptr) }.pivot().map_or(f32::NAN, |p| p.1)
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RenderNodeNative_nSetPivot(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, x: jfloat, y: jfloat) {
    let pivot = if x.is_nan() || y.is_nan() { None } else { Some((x, y)) };
    unsafe { borrow_mut::<RenderNode>(ptr) }.set_pivot(pivot);
}

macro_rules! float_prop {
    ($get_name:ident, $set_name:ident, $getter:ident, $setter:ident) => {
        #[no_mangle]
        pub extern "system" fn $get_name(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
            unsafe { borrow::<RenderNode>(ptr) }.$getter()
        }

        #[no_mangle]
        pub extern "system" fn $set_name(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, value: jfloat) {
            unsafe { borrow_mut::<RenderNode>(ptr) }.$setter(value);
        }
    };
}

float_prop!(Java_org_skialin_RenderNodeNative_nGetAlpha, Java_org_skialin_RenderNodeNative_nSetAlpha, alpha, set_alpha);
float_prop!(Java_org_skialin_RenderNodeNative_nGetScaleX, Java_org_skialin_RenderNodeNative_nSetScaleX, scale_x, set_scale_x);
float_prop!(Java_org_skialin_RenderNodeNative_nGetScaleY, Java_org_skialin_RenderNodeNative_nSetScaleY, scale_y, set_scale_y);
float_prop!(Java_org_skialin_RenderNodeNative_nGetTranslationX, Java_org_skialin_RenderNodeNative_nSetTranslationX, translation_x, set_translation_x);
float_prop!(Java_org_skialin_RenderNodeNative_nGetTranslationY, Java_org_skialin_RenderNodeNative_nSetTranslationY, translation_y, set_translation_y);
float_prop!(Java_org_skialin_RenderNodeNative_nGetShadowElevation, Java_org_skialin_RenderNodeNative_nSetShadowElevation, shadow_elevation, set_shadow_elevation);
float_prop!(Java_org_skialin_RenderNodeNative_nGetRotationX, Java_org_skialin_RenderNodeNative_nSetRotationX, rotation_x, set_rotation_x);
float_prop!(Java_org_skialin_RenderNodeNative_nGetRotationY, Java_org_skialin_RenderNodeNative_nSetRotationY, rotation_y, set_rotation_y);
float_prop!(Java_org_skialin_RenderNodeNative_nGetRotationZ, Java_org_skialin_RenderNodeNative_nSetRotationZ, rotation_z, set_rotation_z);
float_prop!(Java_org_skialin_RenderNodeNative_nGetCameraDistance, Java_org_skialin_RenderNodeNative_nSetCameraDistance, camera_distance, set_camera_distance);

#[no_mangle]
pub extern "system" fn Java_org_skialin_RenderNodeNative_nGetAmbientShadowColor(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<RenderNode>(ptr) }.ambient_shadow_color() as jint
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RenderNodeNative_nSetAmbientShadowColor(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, color: jint) {
    unsafe { borrow_mut::<RenderNode>(ptr) }.set_ambient_shadow_color(color as u32);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RenderNodeNative_nGetSpotShadowColor(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<RenderNode>(ptr) }.spot_shadow_color() as jint
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RenderNodeNative_nSetSpotShadowColor(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, color: jint) {
    unsafe { borrow_mut::<RenderNode>(ptr) }.set_spot_shadow_color(color as u32);
}

#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub extern "system" fn Java_org_skialin_RenderNodeNative_nSetClipRect(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    left: jfloat,
    top: jfloat,
    right: jfloat,
    bottom: jfloat,
    mode: jint,
    antialias: jboolean,
) {
    unsafe { borrow_mut::<RenderNode>(ptr) }.set_clip_rect(Some(Rect::new(left, top, right, bottom)), clip_op_from_jint(mode), antialias != 0);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RenderNodeNative_nSetClipRRect(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, rrect_ptr: jlong, mode: jint, antialias: jboolean) {
    let rrect = unsafe { borrow::<RRect>(rrect_ptr) };
    unsafe { borrow_mut::<RenderNode>(ptr) }.set_clip_rrect(Some(rrect), clip_op_from_jint(mode), antialias != 0);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RenderNodeNative_nSetClipPath(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, path_ptr: jlong, mode: jint, antialias: jboolean) {
    let path = unsafe { borrow::<Path>(path_ptr) };
    unsafe { borrow_mut::<RenderNode>(ptr) }.set_clip_path(Some(path), clip_op_from_jint(mode), antialias != 0);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RenderNodeNative_nGetClip(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<RenderNode>(ptr) }.clip() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RenderNodeNative_nSetClip(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, clip: jboolean) {
    unsafe { borrow_mut::<RenderNode>(ptr) }.set_clip(clip != 0);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RenderNodeNative_nBeginRecording(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    unsafe { borrow_mut::<RenderNode>(ptr) }.begin_recording().as_raw() as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RenderNodeNative_nEndRecording(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { borrow_mut::<RenderNode>(ptr) }.end_recording();
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RenderNodeNative_nDrawInto(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, canvas_ptr: jlong) {
    let node = unsafe { borrow_mut::<RenderNode>(ptr) };
    let mut canvas = unsafe { Canvas::from_raw(canvas_ptr as *mut skialin_core::sys::SkCanvas) };
    node.draw_into(&mut canvas);
}
