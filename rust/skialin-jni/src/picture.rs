use jni::sys::{jboolean, jfloatArray, jint, jlong};
use jni::JNIEnv;

use skialin_core::{Canvas, Picture, PictureRecorder, Rect};

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};

fn rect_from_flat(env: &JNIEnv, rect: jfloatArray) -> Rect {
    let array = unsafe { jni::objects::JFloatArray::from_raw(rect) };
    let mut values = [0f32; 4];
    env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
    Rect::new(values[0], values[1], values[2], values[3])
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PictureRecorderNative_nNew(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    box_ptr(PictureRecorder::new())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PictureRecorderNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<PictureRecorder>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PictureRecorderNative_nBeginRecording(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, bounds: jfloatArray) -> jlong {
    let bounds = rect_from_flat(&env, bounds);
    let canvas = unsafe { borrow_mut::<PictureRecorder>(ptr) }.begin_recording(bounds);
    canvas.as_raw() as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PictureRecorderNative_nGetRecordingCanvas(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    let canvas = unsafe { borrow_mut::<PictureRecorder>(ptr) }.recording_canvas();
    canvas.as_raw() as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PictureRecorderNative_nFinishRecordingAsPicture(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    match unsafe { borrow_mut::<PictureRecorder>(ptr) }.finish_recording_as_picture() {
        Some(picture) => box_ptr(picture),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PictureNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<Picture>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PictureNative_nPlayback(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, canvas_ptr: jlong) {
    let picture = unsafe { borrow::<Picture>(ptr) };
    let mut canvas = unsafe { Canvas::from_raw(canvas_ptr as *mut skialin_core::sys::SkCanvas) };
    picture.playback(&mut canvas);
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PictureNative_nCullRect(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloatArray {
    let rect = unsafe { borrow::<Picture>(ptr) }.cull_rect();
    let array = env.new_float_array(4).expect("new_float_array");
    env.set_float_array_region(&array, 0, &[rect.left, rect.top, rect.right, rect.bottom]).expect("set_float_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PictureNative_nUniqueID(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    unsafe { borrow::<Picture>(ptr) }.unique_id() as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PictureNative_nApproximateOpCount(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, nested: jboolean) -> jint {
    unsafe { borrow::<Picture>(ptr) }.approximate_op_count(nested != 0)
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_CanvasNative_nDrawPicture(_env: JNIEnv, _class: jni::objects::JClass, canvas_ptr: jlong, picture_ptr: jlong) {
    let mut canvas = unsafe { Canvas::from_raw(canvas_ptr as *mut skialin_core::sys::SkCanvas) };
    let picture = unsafe { borrow::<Picture>(picture_ptr) };
    canvas.draw_picture(picture);
}
