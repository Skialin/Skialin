use crate::sys;

/// A 4x5 row-major color matrix, mirroring Skia's `SkColorMatrix`. Feeds
/// directly into [`crate::ColorFilter::matrix`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorMatrix(pub [f32; 20]);

impl ColorMatrix {
    pub fn identity() -> Self {
        let mut m = Self([0.0; 20]);
        unsafe { sys::skialin_bridge_ColorMatrix_setIdentity(m.0.as_mut_ptr()) };
        m
    }

    pub fn scale(r_scale: f32, g_scale: f32, b_scale: f32, a_scale: f32) -> Self {
        let mut m = Self([0.0; 20]);
        unsafe { sys::skialin_bridge_ColorMatrix_setScale(m.0.as_mut_ptr(), r_scale, g_scale, b_scale, a_scale) };
        m
    }

    /// Adjusts saturation: `0` desaturates entirely (grayscale), `1` is a no-op.
    pub fn saturation(sat: f32) -> Self {
        let mut m = Self::identity();
        m.set_saturation(sat);
        m
    }

    pub fn set_saturation(&mut self, sat: f32) {
        unsafe { sys::skialin_bridge_ColorMatrix_setSaturation(self.0.as_mut_ptr(), sat) };
    }

    pub fn post_translate(&mut self, dr: f32, dg: f32, db: f32, da: f32) {
        unsafe { sys::skialin_bridge_ColorMatrix_postTranslate(self.0.as_mut_ptr(), dr, dg, db, da) };
    }

    /// `result = a * b` (`a` is applied after `b`).
    pub fn concat(a: &ColorMatrix, b: &ColorMatrix) -> Self {
        let mut out = Self([0.0; 20]);
        unsafe { sys::skialin_bridge_ColorMatrix_setConcat(out.0.as_mut_ptr(), a.0.as_ptr(), b.0.as_ptr()) };
        out
    }
}

impl Default for ColorMatrix {
    fn default() -> Self {
        Self::identity()
    }
}
