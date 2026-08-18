use crate::{sys, Data};

/// The 7 coefficients (g, a, b, c, d, e, f) of Skia's piecewise transfer
/// function, matching `skcms_TransferFunction`'s layout.
pub type TransferFn = [f32; 7];

/// A row-major 3x3 matrix to XYZ D50, matching `skcms_Matrix3x3`'s layout.
pub type Matrix3x3 = [f32; 9];

pub mod named_transfer_fn {
    use super::TransferFn;

    pub const SRGB: TransferFn = [2.4, 1.0 / 1.055, 0.055 / 1.055, 1.0 / 12.92, 0.04045, 0.0, 0.0];
    pub const TWO_DOT_TWO: TransferFn = [2.2, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    pub const LINEAR: TransferFn = [1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    pub const REC2020: TransferFn = [2.22222, 0.909672, 0.0903276, 0.222222, 0.0812429, 0.0, 0.0];
    pub const PQ: TransferFn = [-5.0, 203.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    pub const HLG: TransferFn = [-6.0, 203.0, 1000.0, 1.2, 0.0, 0.0, 0.0];
}

pub mod named_gamut {
    use super::Matrix3x3;

    #[rustfmt::skip]
    pub const SRGB: Matrix3x3 = [
        0.436065674, 0.385147095, 0.143066406,
        0.222488403, 0.716873169, 0.060607910,
        0.013916016, 0.097076416, 0.714096069,
    ];
    #[rustfmt::skip]
    pub const ADOBE_RGB: Matrix3x3 = [
        0.60974, 0.20528, 0.14919,
        0.31111, 0.62567, 0.06322,
        0.01947, 0.06087, 0.74457,
    ];
    #[rustfmt::skip]
    pub const DISPLAY_P3: Matrix3x3 = [
        0.515102, 0.291965, 0.157153,
        0.241182, 0.692236, 0.0665819,
        -0.00104941, 0.0418818, 0.784378,
    ];
    #[rustfmt::skip]
    pub const REC2020: Matrix3x3 = [
        0.673459, 0.165661, 0.125100,
        0.279033, 0.675338, 0.0456288,
        -0.00193139, 0.0299794, 0.797162,
    ];
    #[rustfmt::skip]
    pub const XYZ: Matrix3x3 = [
        1.0, 0.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 0.0, 1.0,
    ];
}

/// Rows of ITU-T H.273 Table 2 that Skia's `MakeCICP` accepts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CicpPrimaries {
    Rec709 = 1,
    Rec470SystemM = 4,
    Rec470SystemBG = 5,
    Rec601 = 6,
    SmpteSt240 = 7,
    GenericFilm = 8,
    Rec2020 = 9,
    SmpteSt428_1 = 10,
    SmpteRp431_2 = 11,
    SmpteEg432_1 = 12,
    ItuTH273Value22 = 22,
}

/// Rows of ITU-T H.273 Table 3 that Skia's `MakeCICP` accepts.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CicpTransferFn {
    Rec709 = 1,
    Rec470SystemM = 4,
    Rec470SystemBG = 5,
    Rec601 = 6,
    SmpteSt240 = 7,
    Linear = 8,
    Iec61966_2_4 = 11,
    Srgb = 13,
    Rec2020_10bit = 14,
    Rec2020_12bit = 15,
    Pq = 16,
    SmpteSt428_1 = 17,
    Hlg = 18,
}

pub struct ColorSpace(pub(crate) *mut sys::SkColorSpace);

impl ColorSpace {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::SkColorSpace) -> Option<Self> {
        (!ptr.is_null()).then_some(ColorSpace(ptr))
    }

    pub fn srgb() -> Self {
        ColorSpace(unsafe { sys::skialin_bridge_ColorSpace_makeSRGB() })
    }

    pub fn srgb_linear() -> Self {
        ColorSpace(unsafe { sys::skialin_bridge_ColorSpace_makeSRGBLinear() })
    }

    pub fn rgb(transfer_fn: TransferFn, to_xyz_d50: Matrix3x3) -> Self {
        ColorSpace(unsafe { sys::skialin_bridge_ColorSpace_makeRGB(transfer_fn.as_ptr(), to_xyz_d50.as_ptr()) })
    }

    /// `None` for an invalid or unsupported combination of code points.
    pub fn cicp(primaries: CicpPrimaries, transfer_fn: CicpTransferFn) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ColorSpace_makeCICP(primaries as u8, transfer_fn as u8)) }
    }

    /// `None` if `bytes` doesn't parse as an ICC profile.
    pub fn from_icc_profile(bytes: &[u8]) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ColorSpace_makeFromIccProfile(bytes.as_ptr(), bytes.len())) }
    }

    pub fn deserialize(bytes: &[u8]) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_ColorSpace_deserialize(bytes.as_ptr(), bytes.len())) }
    }

    pub fn gamma_close_to_srgb(&self) -> bool {
        unsafe { (*self.0).gammaCloseToSRGB() }
    }

    pub fn gamma_is_linear(&self) -> bool {
        unsafe { (*self.0).gammaIsLinear() }
    }

    pub fn is_srgb(&self) -> bool {
        unsafe { (*self.0).isSRGB() }
    }

    pub fn to_xyz_d50(&self) -> Option<Matrix3x3> {
        let mut out = [0f32; 9];
        unsafe { sys::skialin_bridge_ColorSpace_toXYZD50(self.0, out.as_mut_ptr()) }.then_some(out)
    }

    pub fn to_xyz_d50_hash(&self) -> u32 {
        unsafe { (*self.0).toXYZD50Hash() }
    }

    pub fn transfer_fn(&self) -> TransferFn {
        let mut out = [0f32; 7];
        unsafe { sys::skialin_bridge_ColorSpace_transferFn(self.0, out.as_mut_ptr()) };
        out
    }

    pub fn inv_transfer_fn(&self) -> TransferFn {
        let mut out = [0f32; 7];
        unsafe { sys::skialin_bridge_ColorSpace_invTransferFn(self.0, out.as_mut_ptr()) };
        out
    }

    /// `None` if the transfer function can't be represented by the standard
    /// ICC 7-parameter equation (e.g. PQ, HLG).
    pub fn numerical_transfer_fn(&self) -> Option<TransferFn> {
        let mut out = [0f32; 7];
        unsafe { sys::skialin_bridge_ColorSpace_isNumericalTransferFn(self.0, out.as_mut_ptr()) }.then_some(out)
    }

    pub fn transfer_fn_hash(&self) -> u32 {
        unsafe { (*self.0).transferFnHash() }
    }

    pub fn hash(&self) -> u64 {
        unsafe { (*self.0).hash() }
    }

    pub fn gamut_transform_to(&self, dst: &ColorSpace) -> Matrix3x3 {
        let mut out = [0f32; 9];
        unsafe { sys::skialin_bridge_ColorSpace_gamutTransformTo(self.0, dst.0, out.as_mut_ptr()) };
        out
    }

    pub fn make_linear_gamma(&self) -> ColorSpace {
        ColorSpace(unsafe { sys::skialin_bridge_ColorSpace_makeLinearGamma(self.0) })
    }

    pub fn make_srgb_gamma(&self) -> ColorSpace {
        ColorSpace(unsafe { sys::skialin_bridge_ColorSpace_makeSRGBGamma(self.0) })
    }

    pub fn make_color_spin(&self) -> ColorSpace {
        ColorSpace(unsafe { sys::skialin_bridge_ColorSpace_makeColorSpin(self.0) })
    }

    pub fn serialize(&self) -> Data {
        unsafe { Data::from_raw(sys::skialin_bridge_ColorSpace_serialize(self.0)) }
            .expect("SkColorSpace::serialize never returns null")
    }

    pub fn equals(&self, other: &ColorSpace) -> bool {
        unsafe { sys::skialin_bridge_ColorSpace_equals(self.0, other.0) }
    }
}

impl Drop for ColorSpace {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_ColorSpace_unref(self.0) };
    }
}
