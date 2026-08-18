use jni::sys::{jboolean, jfloat, jint, jlong};
use jni::JNIEnv;

use skialin_core::{PixelGeometry, SurfaceProps, SurfacePropsFlags};

use crate::util::{borrow, box_ptr, drop_ptr};

fn geometry_from_jint(value: jint) -> PixelGeometry {
    match value {
        1 => PixelGeometry::RgbH,
        2 => PixelGeometry::BgrH,
        3 => PixelGeometry::RgbV,
        4 => PixelGeometry::BgrV,
        _ => PixelGeometry::Unknown,
    }
}

fn geometry_to_jint(geometry: PixelGeometry) -> jint {
    match geometry {
        PixelGeometry::Unknown => 0,
        PixelGeometry::RgbH => 1,
        PixelGeometry::BgrH => 2,
        PixelGeometry::RgbV => 3,
        PixelGeometry::BgrV => 4,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SurfacePropsNative_nMake(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    flags: jint,
    pixel_geometry: jint,
    text_contrast: jfloat,
    text_gamma: jfloat,
) -> jlong {
    box_ptr(SurfaceProps::new(SurfacePropsFlags(flags as u32), geometry_from_jint(pixel_geometry), text_contrast, text_gamma))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SurfacePropsNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<SurfaceProps>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SurfacePropsNative_nClone(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    box_ptr(unsafe { borrow::<SurfaceProps>(ptr) }.clone())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SurfacePropsNative_nCloneWithPixelGeometry(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    pixel_geometry: jint,
) -> jlong {
    box_ptr(unsafe { borrow::<SurfaceProps>(ptr) }.clone_with_pixel_geometry(geometry_from_jint(pixel_geometry)))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SurfacePropsNative_nFlags(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<SurfaceProps>(ptr) }.flags().0 as jint
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SurfacePropsNative_nPixelGeometry(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    geometry_to_jint(unsafe { borrow::<SurfaceProps>(ptr) }.pixel_geometry())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SurfacePropsNative_nTextContrast(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<SurfaceProps>(ptr) }.text_contrast()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SurfacePropsNative_nTextGamma(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jfloat {
    unsafe { borrow::<SurfaceProps>(ptr) }.text_gamma()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SurfacePropsNative_nEquals(_env: JNIEnv, _class: jni::objects::JClass, a_ptr: jlong, b_ptr: jlong) -> jboolean {
    (unsafe { borrow::<SurfaceProps>(a_ptr) } == unsafe { borrow::<SurfaceProps>(b_ptr) }) as jboolean
}
