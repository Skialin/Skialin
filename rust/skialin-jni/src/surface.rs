use jni::sys::{jint, jlong};
use jni::JNIEnv;

use skialin_core::{ImageInfo, Surface};

use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};

#[no_mangle]
pub extern "system" fn Java_org_skialin_SurfaceNative_nMakeRasterN32Premul(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    width: jint,
    height: jint,
) -> jlong {
    match Surface::new_raster_n32_premul(width, height) {
        Some(surface) => box_ptr(surface),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SurfaceNative_nMakeRaster(_env: JNIEnv, _class: jni::objects::JClass, info_ptr: jlong) -> jlong {
    let info = unsafe { borrow::<ImageInfo>(info_ptr) };
    match Surface::new_raster(info) {
        Some(surface) => box_ptr(surface),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SurfaceNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<Surface>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SurfaceNative_nGetCanvas(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    unsafe { borrow_mut::<Surface>(ptr) }.canvas().as_raw() as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SurfaceNative_nMakeImageSnapshot(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    match unsafe { borrow_mut::<Surface>(ptr) }.image_snapshot() {
        Some(image) => box_ptr(image),
        None => 0,
    }
}
