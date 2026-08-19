use jni::sys::{jboolean, jint, jintArray, jlong};
use jni::JNIEnv;

use skialin_core::{IRect, Path, Region, RegionOp};

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};

fn region_op_from_jint(v: jint) -> RegionOp {
    match v {
        0 => RegionOp::Difference,
        1 => RegionOp::Intersect,
        2 => RegionOp::Union,
        3 => RegionOp::Xor,
        4 => RegionOp::ReverseDifference,
        _ => RegionOp::Replace,
    }
}

fn irect_from_flat(env: &JNIEnv, rect: jintArray) -> IRect {
    let array = unsafe { jni::objects::JIntArray::from_raw(rect) };
    let mut values = [0i32; 4];
    env.get_int_array_region(&array, 0, &mut values).expect("get_int_array_region");
    IRect::new(values[0], values[1], values[2], values[3])
}

fn irect_to_jintarray(env: &JNIEnv, rect: IRect) -> jintArray {
    let array = env.new_int_array(4).expect("new_int_array");
    env.set_int_array_region(&array, 0, &[rect.left, rect.top, rect.right, rect.bottom]).expect("set_int_array_region");
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RegionNative_nMake(_env: JNIEnv, _class: jni::objects::JClass) -> jlong {
    box_ptr(Region::new())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RegionNative_nMakeRect(env: JNIEnv, _class: jni::objects::JClass, rect: jintArray) -> jlong {
    box_ptr(Region::from_rect(irect_from_flat(&env, rect)))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RegionNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<Region>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RegionNative_nClone(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    box_ptr(unsafe { borrow::<Region>(ptr) }.clone())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RegionNative_nSetRect(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, rect: jintArray) -> jboolean {
    unsafe { borrow_mut::<Region>(ptr) }.set_rect(irect_from_flat(&env, rect)) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RegionNative_nSetPath(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, path_ptr: jlong, clip_ptr: jlong) -> jboolean {
    let path = unsafe { borrow::<Path>(path_ptr) };
    let clip = unsafe { borrow::<Region>(clip_ptr) };
    unsafe { borrow_mut::<Region>(ptr) }.set_path(path, clip) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RegionNative_nOp(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, other_ptr: jlong, op: jint) -> jboolean {
    let other = unsafe { borrow::<Region>(other_ptr) };
    unsafe { borrow_mut::<Region>(ptr) }.op(other, region_op_from_jint(op)) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RegionNative_nOpRect(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, rect: jintArray, op: jint) -> jboolean {
    let rect = irect_from_flat(&env, rect);
    unsafe { borrow_mut::<Region>(ptr) }.op_rect(rect, region_op_from_jint(op)) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RegionNative_nIsEmpty(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Region>(ptr) }.is_empty() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RegionNative_nIsRect(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Region>(ptr) }.is_rect() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RegionNative_nIsComplex(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Region>(ptr) }.is_complex() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RegionNative_nGetBounds(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jintArray {
    irect_to_jintarray(&env, unsafe { borrow::<Region>(ptr) }.bounds())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RegionNative_nContainsPoint(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, x: jint, y: jint) -> jboolean {
    unsafe { borrow::<Region>(ptr) }.contains_point(x, y) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RegionNative_nContainsRect(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, rect: jintArray) -> jboolean {
    unsafe { borrow::<Region>(ptr) }.contains_rect(irect_from_flat(&env, rect)) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RegionNative_nContainsRegion(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, other_ptr: jlong) -> jboolean {
    let other = unsafe { borrow::<Region>(other_ptr) };
    unsafe { borrow::<Region>(ptr) }.contains_region(other) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RegionNative_nIntersectsRect(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, rect: jintArray) -> jboolean {
    unsafe { borrow::<Region>(ptr) }.intersects_rect(irect_from_flat(&env, rect)) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RegionNative_nIntersectsRegion(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, other_ptr: jlong) -> jboolean {
    let other = unsafe { borrow::<Region>(other_ptr) };
    unsafe { borrow::<Region>(ptr) }.intersects_region(other) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RegionNative_nEquals(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, other_ptr: jlong) -> jboolean {
    let other = unsafe { borrow::<Region>(other_ptr) };
    (unsafe { borrow::<Region>(ptr) } == other) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_RegionNative_nGetBoundaryPath(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    box_ptr(unsafe { borrow::<Region>(ptr) }.boundary_path())
}
