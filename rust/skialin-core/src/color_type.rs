use crate::sys;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ColorType {
    Unknown,
    Alpha8,
    Rgb565,
    Argb4444,
    Rgba8888,
    Rgb888x,
    Bgra8888,
    Rgba1010102,
    Bgra1010102,
    Rgb101010x,
    Bgr101010x,
    Bgr101010xXr,
    Bgra10101010Xr,
    Rgba10x6,
    Gray8,
    RgbaF16Norm,
    RgbaF16,
    RgbF16F16F16x,
    RgbaF32,
    R8G8Unorm,
    A16Float,
    R16Float,
    R16G16Float,
    A16Unorm,
    R16Unorm,
    R16G16Unorm,
    R16G16B16A16Unorm,
    Srgba8888,
    R8Unorm,
}

impl ColorType {
    /// The native 32-bit ARGB encoding for this platform: `Bgra8888` on
    /// Windows/most desktop platforms, `Rgba8888` where PMCOLOR order differs.
    pub const N32: ColorType = ColorType::Bgra8888;
}

impl From<ColorType> for sys::SkColorType {
    fn from(ct: ColorType) -> Self {
        use ColorType::*;
        (match ct {
            Unknown => sys::SkColorType_kUnknown_SkColorType,
            Alpha8 => sys::SkColorType_kAlpha_8_SkColorType,
            Rgb565 => sys::SkColorType_kRGB_565_SkColorType,
            Argb4444 => sys::SkColorType_kARGB_4444_SkColorType,
            Rgba8888 => sys::SkColorType_kRGBA_8888_SkColorType,
            Rgb888x => sys::SkColorType_kRGB_888x_SkColorType,
            Bgra8888 => sys::SkColorType_kBGRA_8888_SkColorType,
            Rgba1010102 => sys::SkColorType_kRGBA_1010102_SkColorType,
            Bgra1010102 => sys::SkColorType_kBGRA_1010102_SkColorType,
            Rgb101010x => sys::SkColorType_kRGB_101010x_SkColorType,
            Bgr101010x => sys::SkColorType_kBGR_101010x_SkColorType,
            Bgr101010xXr => sys::SkColorType_kBGR_101010x_XR_SkColorType,
            Bgra10101010Xr => sys::SkColorType_kBGRA_10101010_XR_SkColorType,
            Rgba10x6 => sys::SkColorType_kRGBA_10x6_SkColorType,
            Gray8 => sys::SkColorType_kGray_8_SkColorType,
            RgbaF16Norm => sys::SkColorType_kRGBA_F16Norm_SkColorType,
            RgbaF16 => sys::SkColorType_kRGBA_F16_SkColorType,
            RgbF16F16F16x => sys::SkColorType_kRGB_F16F16F16x_SkColorType,
            RgbaF32 => sys::SkColorType_kRGBA_F32_SkColorType,
            R8G8Unorm => sys::SkColorType_kR8G8_unorm_SkColorType,
            A16Float => sys::SkColorType_kA16_float_SkColorType,
            R16Float => sys::SkColorType_kR16_float_SkColorType,
            R16G16Float => sys::SkColorType_kR16G16_float_SkColorType,
            A16Unorm => sys::SkColorType_kA16_unorm_SkColorType,
            R16Unorm => sys::SkColorType_kR16_unorm_SkColorType,
            R16G16Unorm => sys::SkColorType_kR16G16_unorm_SkColorType,
            R16G16B16A16Unorm => sys::SkColorType_kR16G16B16A16_unorm_SkColorType,
            Srgba8888 => sys::SkColorType_kSRGBA_8888_SkColorType,
            R8Unorm => sys::SkColorType_kR8_unorm_SkColorType,
        }) as sys::SkColorType
    }
}

impl From<sys::SkColorType> for ColorType {
    fn from(ct: sys::SkColorType) -> Self {
        use ColorType::*;
        match ct as u32 {
            1 => Alpha8,
            2 => Rgb565,
            3 => Argb4444,
            4 => Rgba8888,
            5 => Rgb888x,
            6 => Bgra8888,
            7 => Rgba1010102,
            8 => Bgra1010102,
            9 => Rgb101010x,
            10 => Bgr101010x,
            11 => Bgr101010xXr,
            12 => Bgra10101010Xr,
            13 => Rgba10x6,
            14 => Gray8,
            15 => RgbaF16Norm,
            16 => RgbaF16,
            17 => RgbF16F16F16x,
            18 => RgbaF32,
            19 => R8G8Unorm,
            20 => A16Float,
            21 => R16Float,
            22 => R16G16Float,
            23 => A16Unorm,
            24 => R16Unorm,
            25 => R16G16Unorm,
            26 => R16G16B16A16Unorm,
            27 => Srgba8888,
            28 => R8Unorm,
            _ => Unknown,
        }
    }
}
