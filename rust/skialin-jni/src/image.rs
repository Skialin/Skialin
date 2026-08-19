use jni::sys::{jboolean, jbyteArray, jfloatArray, jint, jlong};
use jni::JNIEnv;

use skialin_core::{BackendTexture, Data, DirectContext, FilterMode, GraphiteBackendTexture, GraphiteRecorder, Image, ImageInfo, MipmapMode, Pixmap, SamplingOptions, SurfaceOrigin, TileMode};

use crate::color_space::color_space_ptr_from_jlong;
use crate::color_type::{alpha_type_from_ordinal, alpha_type_to_ordinal, color_type_from_ordinal, color_type_to_ordinal};
use crate::util::{borrow, box_ptr, drop_ptr};

fn tile_mode_from_ordinal(ordinal: jint) -> TileMode {
    match ordinal {
        1 => TileMode::Repeat,
        2 => TileMode::Mirror,
        3 => TileMode::Decal,
        _ => TileMode::Clamp,
    }
}

fn sampling_from_args(max_aniso: jint, use_cubic: jboolean, cubic_b: f32, cubic_c: f32, filter: jint, mipmap: jint) -> SamplingOptions {
    SamplingOptions {
        max_aniso,
        cubic: (use_cubic != 0).then_some((cubic_b, cubic_c)),
        filter: if filter == 1 { FilterMode::Linear } else { FilterMode::Nearest },
        mipmap: match mipmap {
            1 => MipmapMode::Nearest,
            2 => MipmapMode::Linear,
            _ => MipmapMode::None,
        },
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nBufferAddress(env: JNIEnv, _class: jni::objects::JClass, buffer: jni::sys::jobject) -> jlong {
    let buffer = unsafe { jni::objects::JByteBuffer::from_raw(buffer) };
    env.get_direct_buffer_address(&buffer).expect("get_direct_buffer_address") as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nDecode(env: JNIEnv, _class: jni::objects::JClass, bytes: jbyteArray) -> jlong {
    let bytes = unsafe { jni::objects::JByteArray::from_raw(bytes) };
    let len = env.get_array_length(&bytes).expect("get_array_length") as usize;
    let mut buf = vec![0i8; len];
    env.get_byte_array_region(&bytes, 0, &mut buf).expect("get_byte_array_region");
    let buf: Vec<u8> = buf.into_iter().map(|b| b as u8).collect();
    match Image::decode(&buf) {
        Some(image) => box_ptr(image),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nFromPixmapCopy(_env: JNIEnv, _class: jni::objects::JClass, pixmap_ptr: jlong) -> jlong {
    let pixmap = unsafe { borrow::<Pixmap>(pixmap_ptr) };
    match Image::from_pixmap_copy(pixmap) {
        Some(image) => box_ptr(image),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nFromData(_env: JNIEnv, _class: jni::objects::JClass, info_ptr: jlong, data_ptr: jlong, row_bytes: jlong) -> jlong {
    let info = unsafe { borrow::<ImageInfo>(info_ptr) };
    let data = unsafe { borrow::<Data>(data_ptr) };
    match Image::from_data(info, data, row_bytes as usize) {
        Some(image) => box_ptr(image),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nRelease(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) {
    unsafe { drop_ptr::<Image>(ptr) };
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nWidth(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Image>(ptr) }.width()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nHeight(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Image>(ptr) }.height()
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nUniqueId(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    unsafe { borrow::<Image>(ptr) }.unique_id() as jint
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nAlphaType(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    alpha_type_to_ordinal(unsafe { borrow::<Image>(ptr) }.alpha_type())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nColorType(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jint {
    color_type_to_ordinal(unsafe { borrow::<Image>(ptr) }.color_type())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nColorSpace(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    match unsafe { borrow::<Image>(ptr) }.color_space() {
        Some(cs) => box_ptr(cs),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nImageInfo(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    box_ptr(unsafe { borrow::<Image>(ptr) }.image_info())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nIsAlphaOnly(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Image>(ptr) }.is_alpha_only() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nIsOpaque(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Image>(ptr) }.is_opaque() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nIsTextureBacked(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Image>(ptr) }.is_texture_backed() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nIsLazyGenerated(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Image>(ptr) }.is_lazy_generated() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nHasMipmaps(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Image>(ptr) }.has_mipmaps() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nIsProtected(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jboolean {
    unsafe { borrow::<Image>(ptr) }.is_protected() as jboolean
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nMakeShader(
    env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    tile_x: jint,
    tile_y: jint,
    max_aniso: jint,
    use_cubic: jboolean,
    cubic_b: f32,
    cubic_c: f32,
    filter: jint,
    mipmap: jint,
    local_matrix: jfloatArray,
) -> jlong {
    let sampling = sampling_from_args(max_aniso, use_cubic, cubic_b, cubic_c, filter, mipmap);
    let matrix = matrix_from_nullable_array(&env, local_matrix);
    match unsafe { borrow::<Image>(ptr) }.make_shader(tile_mode_from_ordinal(tile_x), tile_mode_from_ordinal(tile_y), sampling, matrix.as_ref()) {
        Some(shader) => box_ptr(shader),
        None => 0,
    }
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nMakeRawShader(
    env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    tile_x: jint,
    tile_y: jint,
    max_aniso: jint,
    use_cubic: jboolean,
    cubic_b: f32,
    cubic_c: f32,
    filter: jint,
    mipmap: jint,
    local_matrix: jfloatArray,
) -> jlong {
    let sampling = sampling_from_args(max_aniso, use_cubic, cubic_b, cubic_c, filter, mipmap);
    let matrix = matrix_from_nullable_array(&env, local_matrix);
    match unsafe { borrow::<Image>(ptr) }.make_raw_shader(tile_mode_from_ordinal(tile_x), tile_mode_from_ordinal(tile_y), sampling, matrix.as_ref()) {
        Some(shader) => box_ptr(shader),
        None => 0,
    }
}

fn matrix_from_nullable_array(env: &JNIEnv, array: jfloatArray) -> Option<skialin_core::Matrix> {
    if array.is_null() {
        return None;
    }
    let array = unsafe { jni::objects::JFloatArray::from_raw(array) };
    let mut values = [0f32; 9];
    env.get_float_array_region(&array, 0, &mut values).expect("get_float_array_region");
    Some(skialin_core::Matrix::from_array(values))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nPeekPixels(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    match unsafe { borrow::<Image>(ptr) }.peek_pixels() {
        Some(pixmap) => box_ptr(pixmap),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nReadPixels(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    dst_info_ptr: jlong,
    dst_addr: jlong,
    dst_row_bytes: jlong,
    src_x: jint,
    src_y: jint,
) -> jboolean {
    let info = unsafe { borrow::<ImageInfo>(dst_info_ptr) };
    unsafe { borrow::<Image>(ptr).read_pixels(info, dst_addr as *mut u8, dst_row_bytes as usize, src_x, src_y) as jboolean }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nScalePixels(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    dst_pixmap_ptr: jlong,
    max_aniso: jint,
    use_cubic: jboolean,
    cubic_b: f32,
    cubic_c: f32,
    filter: jint,
    mipmap: jint,
) -> jboolean {
    let sampling = sampling_from_args(max_aniso, use_cubic, cubic_b, cubic_c, filter, mipmap);
    let dst = unsafe { borrow_mut_pixmap(dst_pixmap_ptr) };
    unsafe { borrow::<Image>(ptr) }.scale_pixels(dst, sampling) as jboolean
}

/// # Safety
/// `ptr` must be a live `Pixmap` pointer from `box_ptr::<Pixmap>`.
unsafe fn borrow_mut_pixmap<'a>(ptr: jlong) -> &'a mut Pixmap {
    &mut *(ptr as *mut Pixmap)
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nMakeScaled(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    info_ptr: jlong,
    max_aniso: jint,
    use_cubic: jboolean,
    cubic_b: f32,
    cubic_c: f32,
    filter: jint,
    mipmap: jint,
) -> jlong {
    let sampling = sampling_from_args(max_aniso, use_cubic, cubic_b, cubic_c, filter, mipmap);
    let info = unsafe { borrow::<ImageInfo>(info_ptr) };
    match unsafe { borrow::<Image>(ptr) }.make_scaled(info, sampling) {
        Some(image) => box_ptr(image),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nRefEncodedData(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    match unsafe { borrow::<Image>(ptr) }.ref_encoded_data() {
        Some(data) => box_ptr(data),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nEncodeToPng(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jbyteArray {
    bytes_or_null(&env, unsafe { borrow::<Image>(ptr) }.encode_to_png())
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nEncodeToJpeg(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, quality: jint) -> jbyteArray {
    bytes_or_null(&env, unsafe { borrow::<Image>(ptr) }.encode_to_jpeg(quality))
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nEncodeToWebp(env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, quality: jni::sys::jfloat, lossless: jboolean) -> jbyteArray {
    bytes_or_null(&env, unsafe { borrow::<Image>(ptr) }.encode_to_webp(quality, lossless != 0))
}

fn bytes_or_null(env: &JNIEnv, bytes: Option<Vec<u8>>) -> jbyteArray {
    match bytes {
        Some(bytes) => {
            let array = env.new_byte_array(bytes.len() as i32).expect("new_byte_array");
            let signed: Vec<i8> = bytes.into_iter().map(|b| b as i8).collect();
            env.set_byte_array_region(&array, 0, &signed).expect("set_byte_array_region");
            array.into_raw()
        }
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nMakeSubset(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    left: jint,
    top: jint,
    right: jint,
    bottom: jint,
    mipmapped: jboolean,
) -> jlong {
    let subset = skialin_core::IRect::new(left, top, right, bottom);
    match unsafe { borrow::<Image>(ptr) }.make_subset(subset, mipmapped != 0) {
        Some(image) => box_ptr(image),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nWithDefaultMipmaps(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    match unsafe { borrow::<Image>(ptr) }.with_default_mipmaps() {
        Some(image) => box_ptr(image),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nMakeNonTextureImage(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    match unsafe { borrow::<Image>(ptr) }.make_non_texture_image() {
        Some(image) => box_ptr(image),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nMakeRasterImage(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, allow_caching: jboolean) -> jlong {
    match unsafe { borrow::<Image>(ptr) }.make_raster_image(allow_caching != 0) {
        Some(image) => box_ptr(image),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nAsLegacyBitmap(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong) -> jlong {
    match unsafe { borrow::<Image>(ptr) }.as_legacy_bitmap() {
        Some(bitmap) => box_ptr(bitmap),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nMakeColorSpace(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, target_ptr: jlong, mipmapped: jboolean) -> jlong {
    let target = match color_space_ptr_from_jlong(target_ptr) {
        Some(cs) => cs,
        None => return 0,
    };
    match unsafe { borrow::<Image>(ptr) }.make_color_space(target, mipmapped != 0) {
        Some(image) => box_ptr(image),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nMakeColorTypeAndColorSpace(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    ptr: jlong,
    target_color_type: jint,
    target_cs_ptr: jlong,
    mipmapped: jboolean,
) -> jlong {
    let target_cs = match color_space_ptr_from_jlong(target_cs_ptr) {
        Some(cs) => cs,
        None => return 0,
    };
    match unsafe { borrow::<Image>(ptr) }.make_color_type_and_color_space(color_type_from_ordinal(target_color_type), target_cs, mipmapped != 0) {
        Some(image) => box_ptr(image),
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_org_skialin_ImageNative_nReinterpretColorSpace(_env: JNIEnv, _class: jni::objects::JClass, ptr: jlong, target_ptr: jlong) -> jlong {
    let target = match color_space_ptr_from_jlong(target_ptr) {
        Some(cs) => cs,
        None => return 0,
    };
    match unsafe { borrow::<Image>(ptr) }.reinterpret_color_space(target) {
        Some(image) => box_ptr(image),
        None => 0,
    }
}

#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub extern "system" fn Java_org_skialin_ImageNative_nAdoptTextureFrom(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    context_ptr: jlong,
    backend_texture_ptr: jlong,
    texture_origin: jint,
    color_type: jint,
    alpha_type: jint,
    color_space_ptr: jlong,
) -> jlong {
    let context = unsafe { crate::util::borrow_mut::<DirectContext>(context_ptr) };
    let backend_texture = unsafe { borrow::<BackendTexture>(backend_texture_ptr) };
    let origin = if texture_origin == 0 { SurfaceOrigin::TopLeft } else { SurfaceOrigin::BottomLeft };
    let color_space = color_space_ptr_from_jlong(color_space_ptr);
    match Image::adopt_texture_from(context, backend_texture, origin, color_type_from_ordinal(color_type), alpha_type_from_ordinal(alpha_type), color_space) {
        Some(image) => box_ptr(image),
        None => 0,
    }
}

#[no_mangle]
#[allow(clippy::too_many_arguments)]
pub extern "system" fn Java_org_skialin_ImageNative_nWrapGraphiteTexture(
    _env: JNIEnv,
    _class: jni::objects::JClass,
    recorder_ptr: jlong,
    backend_texture_ptr: jlong,
    alpha_type: jint,
    color_space_ptr: jlong,
    origin: jint,
    generate_mipmaps_from_base: jboolean,
) -> jlong {
    let recorder = unsafe { crate::util::borrow_mut::<GraphiteRecorder>(recorder_ptr) };
    let backend_texture = unsafe { borrow::<GraphiteBackendTexture>(backend_texture_ptr) };
    let color_space = color_space_ptr_from_jlong(color_space_ptr);
    let origin = if origin == 0 { SurfaceOrigin::TopLeft } else { SurfaceOrigin::BottomLeft };
    match Image::wrap_graphite_texture(recorder, backend_texture, alpha_type_from_ordinal(alpha_type), color_space, origin, generate_mipmaps_from_base != 0) {
        Some(image) => box_ptr(image),
        None => 0,
    }
}
