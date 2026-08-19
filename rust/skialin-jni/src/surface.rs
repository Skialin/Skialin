use jni::sys::{jboolean, jint, jlong};
use jni::JNIEnv;

use skialin_core::{
    BackendTexture, ColorSpace, ColorType, DirectContext, GraphiteBackendTexture, GraphiteRecorder, ImageInfo, Surface, SurfaceOrigin,
    SurfaceProps,
};

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
pub extern "system" fn Java_org_skialin_SurfaceNative_nMakeRenderTarget(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    context_ptr: jlong,
    budgeted: jboolean,
    info_ptr: jlong,
    sample_count: jint,
    surface_origin: jint,
    surface_props_ptr: jlong,
    should_create_with_mips: jboolean,
    is_protected: jboolean,
) -> jlong {
    let context = unsafe { borrow_mut::<DirectContext>(context_ptr) };
    let info = unsafe { borrow::<ImageInfo>(info_ptr) };
    let origin = if surface_origin == 0 { SurfaceOrigin::TopLeft } else { SurfaceOrigin::BottomLeft };
    let surface_props = (surface_props_ptr != 0).then(|| unsafe { borrow::<SurfaceProps>(surface_props_ptr) });
    match Surface::new_render_target(
        context,
        budgeted != 0,
        info,
        sample_count,
        origin,
        surface_props,
        should_create_with_mips != 0,
        is_protected != 0,
    ) {
        Some(surface) => box_ptr(surface),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SurfaceNative_nWrapBackendTexture(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    context_ptr: jlong,
    backend_texture_ptr: jlong,
    origin: jint,
    sample_cnt: jint,
    color_type: jint,
    color_space_ptr: jlong,
    surface_props_ptr: jlong,
) -> jlong {
    let context = unsafe { borrow_mut::<DirectContext>(context_ptr) };
    let backend_texture = unsafe { borrow::<BackendTexture>(backend_texture_ptr) };
    let origin = if origin == 0 { SurfaceOrigin::TopLeft } else { SurfaceOrigin::BottomLeft };
    let color_type = crate::color_type::color_type_from_ordinal(color_type);
    let color_space = (color_space_ptr != 0).then(|| unsafe { borrow::<ColorSpace>(color_space_ptr) });
    let surface_props = (surface_props_ptr != 0).then(|| unsafe { borrow::<SurfaceProps>(surface_props_ptr) });
    match Surface::wrap_backend_texture(context, backend_texture, origin, sample_cnt, color_type, color_space, surface_props) {
        Some(surface) => box_ptr(surface),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SurfaceNative_nMakeGraphiteRenderTarget(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    recorder_ptr: jlong,
    info_ptr: jlong,
    mipmapped: jboolean,
    surface_props_ptr: jlong,
) -> jlong {
    let recorder = unsafe { borrow_mut::<GraphiteRecorder>(recorder_ptr) };
    let info = unsafe { borrow::<ImageInfo>(info_ptr) };
    let surface_props = (surface_props_ptr != 0).then(|| unsafe { borrow::<SurfaceProps>(surface_props_ptr) });
    match Surface::new_graphite_render_target(recorder, info, mipmapped != 0, surface_props) {
        Some(surface) => box_ptr(surface),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_SurfaceNative_nWrapGraphiteBackendTexture(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    recorder_ptr: jlong,
    backend_texture_ptr: jlong,
    color_type: jint,
    color_space_ptr: jlong,
    surface_props_ptr: jlong,
) -> jlong {
    let recorder = unsafe { borrow_mut::<GraphiteRecorder>(recorder_ptr) };
    let backend_texture = unsafe { borrow::<GraphiteBackendTexture>(backend_texture_ptr) };
    let color_type = crate::color_type::color_type_from_ordinal(color_type);
    let color_space = (color_space_ptr != 0).then(|| unsafe { borrow::<ColorSpace>(color_space_ptr) });
    let surface_props = (surface_props_ptr != 0).then(|| unsafe { borrow::<SurfaceProps>(surface_props_ptr) });
    match Surface::wrap_graphite_backend_texture(recorder, backend_texture, color_type, color_space, surface_props) {
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
