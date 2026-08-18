use jni::sys::{jboolean, jfloat, jfloatArray, jlong};
use jni::JNIEnv;

use skialin_core::{Point, RRect, Rect};

use crate::util::{borrow, box_ptr, drop_ptr};

fn rect_from_flat(env: &JNIEnv, rect: jfloatArray) -> Rect {
    let array = unsafe { jni::objects::JFloatArray::from_raw(rect) };
    let mut values = [0f32; 4];
    env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
    Rect::new(values[0], values[1], values[2], values[3])
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RRectNative_nMakeRect(env: JNIEnv, _class: jni::objects::JClass, rect: jfloatArray) -> jlong {
    box_ptr(RRect::make_rect(rect_from_flat(&env, rect)))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RRectNative_nMakeOval(env: JNIEnv, _class: jni::objects::JClass, oval: jfloatArray) -> jlong {
    box_ptr(RRect::make_oval(rect_from_flat(&env, oval)))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RRectNative_nMakeRectXY(env: JNIEnv, _class: jni::objects::JClass, rect: jfloatArray, x_rad: jfloat, y_rad: jfloat) -> jlong {
    box_ptr(RRect::make_rect_xy(rect_from_flat(&env, rect), x_rad, y_rad))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RRectNative_nMakeRectRadii(env: JNIEnv, _class: jni::objects::JClass, rect: jfloatArray, radii: jfloatArray) -> jlong {
    let rect = rect_from_flat(&env, rect);
    let array = unsafe { jni::objects::JFloatArray::from_raw(radii) };
    let mut values = [0f32; 8];
    env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
    let corners = [(values[0], values[1]), (values[2], values[3]), (values[4], values[5]), (values[6], values[7])];
    box_ptr(RRect::make_rect_radii(rect, corners))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RRectNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<RRect>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RRectNative_nClone(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    box_ptr(unsafe { borrow::<RRect>(ptr) }.clone())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RRectNative_nRect(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloatArray {
    let rect = unsafe { borrow::<RRect>(ptr) }.rect();
    let array = env.new_float_array(4).expect("new_float_array");
    env.set_float_array_region(&array, 0, &[rect.left, rect.top, rect.right, rect.bottom]).expect("set_float_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RRectNative_nRadii(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloatArray {
    let radii = unsafe { borrow::<RRect>(ptr) }.radii();
    let flat: Vec<f32> = radii.iter().flat_map(|&(x, y)| [x, y]).collect();
    let array = env.new_float_array(8).expect("new_float_array");
    env.set_float_array_region(&array, 0, &flat).expect("set_float_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RRectNative_nType(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jni::sys::jint {
    unsafe { borrow::<RRect>(ptr) }.rrect_type() as jni::sys::jint
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RRectNative_nContainsPoint(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, x: jfloat, y: jfloat) -> jboolean {
    unsafe { borrow::<RRect>(ptr) }.contains_point(Point::new(x, y)) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RRectNative_nContainsRect(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, rect: jfloatArray) -> jboolean {
    unsafe { borrow::<RRect>(ptr) }.contains_rect(rect_from_flat(&env, rect)) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RRectNative_nIsValid(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<RRect>(ptr) }.is_valid() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RRectNative_nInset(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, dx: jfloat, dy: jfloat) -> jlong {
    box_ptr(unsafe { borrow::<RRect>(ptr) }.inset(dx, dy))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RRectNative_nOutset(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, dx: jfloat, dy: jfloat) -> jlong {
    box_ptr(unsafe { borrow::<RRect>(ptr) }.outset(dx, dy))
}
