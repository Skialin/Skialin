use crate::sys;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelGeometry {
    Unknown,
    RgbH,
    BgrH,
    RgbV,
    BgrV,
}

impl From<PixelGeometry> for sys::SkPixelGeometry {
    fn from(geometry: PixelGeometry) -> Self {
        match geometry {
            PixelGeometry::Unknown => sys::SkPixelGeometry_kUnknown_SkPixelGeometry,
            PixelGeometry::RgbH => sys::SkPixelGeometry_kRGB_H_SkPixelGeometry,
            PixelGeometry::BgrH => sys::SkPixelGeometry_kBGR_H_SkPixelGeometry,
            PixelGeometry::RgbV => sys::SkPixelGeometry_kRGB_V_SkPixelGeometry,
            PixelGeometry::BgrV => sys::SkPixelGeometry_kBGR_V_SkPixelGeometry,
        }
    }
}

impl From<sys::SkPixelGeometry> for PixelGeometry {
    fn from(value: sys::SkPixelGeometry) -> Self {
        match value {
            sys::SkPixelGeometry_kRGB_H_SkPixelGeometry => PixelGeometry::RgbH,
            sys::SkPixelGeometry_kBGR_H_SkPixelGeometry => PixelGeometry::BgrH,
            sys::SkPixelGeometry_kRGB_V_SkPixelGeometry => PixelGeometry::RgbV,
            sys::SkPixelGeometry_kBGR_V_SkPixelGeometry => PixelGeometry::BgrV,
            _ => PixelGeometry::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SurfacePropsFlags(pub u32);

impl SurfacePropsFlags {
    pub const DEFAULT: Self = Self(0);
    pub const USE_DEVICE_INDEPENDENT_FONTS: Self = Self(1 << 0);
    pub const DYNAMIC_MSAA: Self = Self(1 << 1);
    pub const ALWAYS_DITHER: Self = Self(1 << 2);
    pub const PRESERVES_TRANSPARENT_DRAWS: Self = Self(1 << 3);
}

impl std::ops::BitOr for SurfacePropsFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

pub struct SurfaceProps(pub(crate) *mut sys::SkSurfaceProps);

impl SurfaceProps {
    pub fn new(flags: SurfacePropsFlags, pixel_geometry: PixelGeometry, text_contrast: f32, text_gamma: f32) -> Self {
        let ptr = unsafe { sys::skialin_bridge_SurfaceProps_make(flags.0, pixel_geometry.into(), text_contrast, text_gamma) };
        SurfaceProps(ptr)
    }

    pub fn clone_with_pixel_geometry(&self, pixel_geometry: PixelGeometry) -> SurfaceProps {
        let ptr = unsafe { sys::skialin_bridge_SurfaceProps_cloneWithPixelGeometry(self.0, pixel_geometry.into()) };
        SurfaceProps(ptr)
    }

    pub fn flags(&self) -> SurfacePropsFlags {
        SurfacePropsFlags(unsafe { sys::skialin_bridge_SurfaceProps_flags(self.0) })
    }

    pub fn pixel_geometry(&self) -> PixelGeometry {
        unsafe { sys::skialin_bridge_SurfaceProps_pixelGeometry(self.0) }.into()
    }

    pub fn text_contrast(&self) -> f32 {
        unsafe { sys::skialin_bridge_SurfaceProps_textContrast(self.0) }
    }

    pub fn text_gamma(&self) -> f32 {
        unsafe { sys::skialin_bridge_SurfaceProps_textGamma(self.0) }
    }
}

impl Clone for SurfaceProps {
    fn clone(&self) -> Self {
        SurfaceProps(unsafe { sys::skialin_bridge_SurfaceProps_clone(self.0) })
    }
}

impl PartialEq for SurfaceProps {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sys::skialin_bridge_SurfaceProps_equals(self.0, other.0) }
    }
}

impl Drop for SurfaceProps {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_SurfaceProps_delete(self.0) };
    }
}
