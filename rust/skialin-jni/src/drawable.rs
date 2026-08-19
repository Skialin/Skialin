use std::os::raw::c_void;
use std::sync::{Arc, OnceLock};

use jni::objects::{GlobalRef, JObject, JValue};
use jni::sys::jlong;
use jni::{JNIEnv, JavaVM};

use skialin_core::sys;
use skialin_core::{Canvas, Drawable, Matrix, Rect};

use crate::util::{borrow, box_ptr, drop_ptr};

/// Shared between the Kotlin-facing [`JniDrawable`] and the C++-side
/// `SkDrawable`'s opaque context: the two have independent lifetimes (Skia
/// may keep the `SkDrawable` alive, e.g. inside a recorded `Picture`, after
/// the Kotlin wrapper is closed/collected), so this can't live inside
/// either one alone.
struct JniDrawableContext {
    jvm: JavaVM,
    // Set once, shortly after construction, from `Drawable`'s `init {}`
    // block once `this` exists -- see `nBindCallback`.
    callback: OnceLock<GlobalRef>,
}

struct JniDrawable {
    drawable: Drawable,
    context: Arc<JniDrawableContext>,
}

unsafe extern "C" fn on_draw(context: *mut c_void, canvas: *mut sys::SkCanvas) {
    let ctx = unsafe { &*(context as *const JniDrawableContext) };
    let (Some(callback), Ok(mut env)) = (ctx.callback.get(), ctx.jvm.attach_current_thread()) else {
        return;
    };
    let canvas_ptr = unsafe { Canvas::from_raw(canvas) }.as_raw() as jlong;
    let _ = env.call_method(callback.as_obj(), "onDrawNative", "(J)V", &[JValue::Long(canvas_ptr)]);
}

unsafe extern "C" fn on_get_bounds(context: *mut c_void, out_bounds: *mut sys::SkRect) {
    let ctx = unsafe { &*(context as *const JniDrawableContext) };
    let bounds = read_bounds(ctx).unwrap_or(Rect::new(0.0, 0.0, 0.0, 0.0));
    unsafe { *out_bounds = bounds.into() };
}

fn read_bounds(ctx: &JniDrawableContext) -> Option<Rect> {
    let callback = ctx.callback.get()?;
    let mut env = ctx.jvm.attach_current_thread().ok()?;
    let result = env.call_method(callback.as_obj(), "onGetBoundsNative", "()[F", &[]).ok()?.l().ok()?;
    let array = jni::objects::JFloatArray::from(result);
    let mut buf = [0f32; 4];
    env.get_float_array_region(&array, 0, &mut buf).ok()?;
    Some(Rect::new(buf[0], buf[1], buf[2], buf[3]))
}

unsafe extern "C" fn on_dispose(context: *mut c_void) {
    unsafe { drop(Arc::from_raw(context as *const JniDrawableContext)) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DrawableNative_nMake(env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    let jvm = env.get_java_vm().expect("get_java_vm");
    let context = Arc::new(JniDrawableContext { jvm, callback: OnceLock::new() });
    let context_ptr = Arc::into_raw(context.clone()) as *mut c_void;
    let raw = unsafe { sys::skialin_bridge_Drawable_Make(context_ptr, Some(on_draw), Some(on_get_bounds), Some(on_dispose)) };
    box_ptr(JniDrawable { drawable: unsafe { Drawable::from_raw(raw) }, context })
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_DrawableNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<JniDrawable>(ptr) };
}

/// Called once from Kotlin's `Drawable.init {}`, after `this` exists, to
/// hand the native side a durable reference to call back into.
#[no_mangle]
pub extern "system" fn Java_org_skialin_DrawableNative_nBindCallback<'l>(env: JNIEnv<'l>, _class: jni::objects::JClass<'l>, ptr: jlong, self_obj: JObject<'l>) {
    let jni_drawable = unsafe { borrow::<JniDrawable>(ptr) };
    let global_ref = env.new_global_ref(self_obj).expect("new_global_ref");
    let _ = jni_drawable.context.callback.set(global_ref);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawDrawable(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, drawable_ptr: jlong, matrix: jni::sys::jfloatArray) {
    let jni_drawable = unsafe { borrow::<JniDrawable>(drawable_ptr) };
    let matrix = if matrix.is_null() {
        None
    } else {
        let array = unsafe { jni::objects::JFloatArray::from_raw(matrix) };
        let mut values = [0f32; 9];
        env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
        Some(Matrix::from_array(values))
    };
    let mut canvas = unsafe { Canvas::from_raw(ptr as *mut sys::SkCanvas) };
    canvas.draw_drawable(&jni_drawable.drawable, matrix.as_ref());
}
