use jni::sys::jint;

use skialin_core::{AlphaType, ColorType};

/// Ordinal matches `org.skialin.ColorType`'s declaration order, which
/// mirrors `SkColorType`.
pub(crate) fn color_type_from_ordinal(ordinal: jint) -> ColorType {
    use ColorType::*;
    const VARIANTS: [ColorType; 29] = [
        Unknown, Alpha8, Rgb565, Argb4444, Rgba8888, Rgb888x, Bgra8888, Rgba1010102, Bgra1010102, Rgb101010x,
        Bgr101010x, Bgr101010xXr, Bgra10101010Xr, Rgba10x6, Gray8, RgbaF16Norm, RgbaF16, RgbF16F16F16x, RgbaF32,
        R8G8Unorm, A16Float, R16Float, R16G16Float, A16Unorm, R16Unorm, R16G16Unorm, R16G16B16A16Unorm, Srgba8888,
        R8Unorm,
    ];
    VARIANTS.get(ordinal as usize).copied().unwrap_or(Unknown)
}

pub(crate) fn color_type_to_ordinal(color_type: ColorType) -> jint {
    use ColorType::*;
    (match color_type {
        Unknown => 0,
        Alpha8 => 1,
        Rgb565 => 2,
        Argb4444 => 3,
        Rgba8888 => 4,
        Rgb888x => 5,
        Bgra8888 => 6,
        Rgba1010102 => 7,
        Bgra1010102 => 8,
        Rgb101010x => 9,
        Bgr101010x => 10,
        Bgr101010xXr => 11,
        Bgra10101010Xr => 12,
        Rgba10x6 => 13,
        Gray8 => 14,
        RgbaF16Norm => 15,
        RgbaF16 => 16,
        RgbF16F16F16x => 17,
        RgbaF32 => 18,
        R8G8Unorm => 19,
        A16Float => 20,
        R16Float => 21,
        R16G16Float => 22,
        A16Unorm => 23,
        R16Unorm => 24,
        R16G16Unorm => 25,
        R16G16B16A16Unorm => 26,
        Srgba8888 => 27,
        R8Unorm => 28,
    }) as jint
}

/// Ordinal matches `org.skialin.AlphaType`'s declaration order, which
/// mirrors `SkAlphaType`.
pub(crate) fn alpha_type_from_ordinal(ordinal: jint) -> AlphaType {
    match ordinal {
        1 => AlphaType::Opaque,
        2 => AlphaType::Premul,
        3 => AlphaType::Unpremul,
        _ => AlphaType::Unknown,
    }
}

pub(crate) fn alpha_type_to_ordinal(alpha_type: AlphaType) -> jint {
    match alpha_type {
        AlphaType::Unknown => 0,
        AlphaType::Opaque => 1,
        AlphaType::Premul => 2,
        AlphaType::Unpremul => 3,
    }
}
