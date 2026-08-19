use jni::objects::JByteBuffer;
use jni::sys::{jboolean, jfloat, jint, jlong, jobject};
use jni::JNIEnv;

use skialin_core::{ImageInfo, Pixmap};

use crate::color_type::{alpha_type_to_ordinal, color_type_to_ordinal};
use crate::util::{borrow, borrow_mut, box_ptr, drop_ptr};

fn sampling_from_args(max_aniso: jint, use_cubic: jboolean, cubic_b: f32, cubic_c: f32, filter: jint, mipmap: jint) -> skialin_core::SamplingOptions {
    skialin_core::SamplingOptions {
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
pub extern "system" fn Java_org_skialin_PixmapNative_nBufferAddress(env: JNIEnv, _class: jni::objects::JClass, buffer: jobject) -> jlong {
    let buffer = unsafe { JByteBuffer::from_raw(buffer) };
    env.get_direct_buffer_address(&buffer).expect("get_direct_buffer_address") as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PixmapNative_nMake(_env: JNIEnv, _class: jni::objects::JClass, info_ptr: jlong, addr: jlong, row_bytes: jlong) -> jlong {
    let info = unsafe { borrow::<ImageInfo>(info_ptr) };
    box_ptr(unsafe { Pixmap::new(info, addr as *const u8, row_bytes as usize) })
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PixmapNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<Pixmap>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PixmapNative_nAddr(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    unsafe { borrow::<Pixmap>(ptr) }.addr() as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PixmapNative_nRowBytes(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    unsafe { borrow::<Pixmap>(ptr) }.row_bytes() as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PixmapNative_nWidth(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Pixmap>(ptr) }.width()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PixmapNative_nHeight(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Pixmap>(ptr) }.height()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PixmapNative_nIsEmpty(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Pixmap>(ptr) }.is_empty() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PixmapNative_nColorType(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    color_type_to_ordinal(unsafe { borrow::<Pixmap>(ptr) }.color_type())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PixmapNative_nAlphaType(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    alpha_type_to_ordinal(unsafe { borrow::<Pixmap>(ptr) }.alpha_type())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PixmapNative_nIsOpaque(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Pixmap>(ptr) }.is_opaque() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PixmapNative_nRowBytesAsPixels(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Pixmap>(ptr) }.row_bytes_as_pixels()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PixmapNative_nShiftPerPixel(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Pixmap>(ptr) }.shift_per_pixel()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PixmapNative_nComputeByteSize(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    unsafe { borrow::<Pixmap>(ptr) }.compute_byte_size() as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PixmapNative_nGetColor(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, x: jint, y: jint) -> jint {
    unsafe { borrow::<Pixmap>(ptr) }.get_color(x, y) as jint
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PixmapNative_nGetAlphaf(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, x: jint, y: jint) -> jfloat {
    unsafe { borrow::<Pixmap>(ptr) }.get_alphaf(x, y)
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PixmapNative_nExtractSubset(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    left: jint,
    top: jint,
    right: jint,
    bottom: jint,
) -> jlong {
    let area = skialin_core::IRect::new(left, top, right, bottom);
    match unsafe { borrow::<Pixmap>(ptr).extract_subset(area) } {
        Some(subset) => box_ptr(subset),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_PixmapNative_nReadPixels(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, dst_ptr: jlong, src_x: jint, src_y: jint) -> jboolean {
    let dst = unsafe { borrow_mut::<Pixmap>(dst_ptr) };
    unsafe { borrow::<Pixmap>(ptr) }.read_pixels(dst, src_x, src_y) as jboolean
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_PixmapNative_nScalePixels(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    dst_ptr: jlong,
    max_aniso: jint,
    use_cubic: jboolean,
    cubic_b: jfloat,
    cubic_c: jfloat,
    filter: jint,
    mipmap: jint,
) -> jboolean {
    let dst = unsafe { borrow_mut::<Pixmap>(dst_ptr) };
    let sampling = sampling_from_args(max_aniso, use_cubic, cubic_b, cubic_c, filter, mipmap);
    unsafe { borrow::<Pixmap>(ptr) }.scale_pixels(dst, sampling) as jboolean
}
