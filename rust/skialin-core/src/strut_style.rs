use crate::{sys, FontStyle};

pub struct StrutStyle(pub(crate) *mut sys::skia::textlayout::StrutStyle);

impl StrutStyle {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::skia::textlayout::StrutStyle) -> Self {
        StrutStyle(ptr)
    }

    pub fn new() -> Self {
        unsafe { Self::from_raw(sys::skialin_bridge_StrutStyle_new()) }
    }

    pub fn font_families(&self) -> Vec<String> {
        let count = unsafe { sys::skialin_bridge_StrutStyle_countFontFamilies(self.0) };
        (0..count)
            .map(|i| {
                let data = unsafe { crate::Data::from_raw(sys::skialin_bridge_StrutStyle_fontFamily(self.0, i)) }.expect("fontFamily never returns null");
                String::from_utf8_lossy(data.as_bytes()).into_owned()
            })
            .collect()
    }

    pub fn set_font_families(&mut self, families: &[&str]) {
        let ptrs: Vec<*const std::os::raw::c_char> = families.iter().map(|s| s.as_ptr().cast()).collect();
        let lens: Vec<usize> = families.iter().map(|s| s.len()).collect();
        unsafe { sys::skialin_bridge_StrutStyle_setFontFamilies(self.0, ptrs.as_ptr(), lens.as_ptr(), families.len()) };
    }

    pub fn font_style(&self) -> FontStyle {
        let (mut weight, mut width, mut slant) = (0, 0, 0);
        unsafe { sys::skialin_bridge_StrutStyle_getFontStyle(self.0, &mut weight, &mut width, &mut slant) };
        FontStyle::new(weight, width, slant.into())
    }

    pub fn set_font_style(&mut self, style: FontStyle) {
        unsafe { sys::skialin_bridge_StrutStyle_setFontStyle(self.0, style.weight, style.width, style.slant.into()) };
    }

    pub fn font_size(&self) -> f32 {
        unsafe { sys::skialin_bridge_StrutStyle_getFontSize(self.0) }
    }

    pub fn set_font_size(&mut self, size: f32) {
        unsafe { sys::skialin_bridge_StrutStyle_setFontSize(self.0, size) };
    }

    pub fn height(&self) -> f32 {
        unsafe { sys::skialin_bridge_StrutStyle_getHeight(self.0) }
    }

    pub fn set_height(&mut self, height: f32) {
        unsafe { sys::skialin_bridge_StrutStyle_setHeight(self.0, height) };
    }

    pub fn leading(&self) -> f32 {
        unsafe { sys::skialin_bridge_StrutStyle_getLeading(self.0) }
    }

    pub fn set_leading(&mut self, leading: f32) {
        unsafe { sys::skialin_bridge_StrutStyle_setLeading(self.0, leading) };
    }

    pub fn strut_enabled(&self) -> bool {
        unsafe { sys::skialin_bridge_StrutStyle_getStrutEnabled(self.0) }
    }

    pub fn set_strut_enabled(&mut self, enabled: bool) {
        unsafe { sys::skialin_bridge_StrutStyle_setStrutEnabled(self.0, enabled) };
    }

    pub fn force_strut_height(&self) -> bool {
        unsafe { sys::skialin_bridge_StrutStyle_getForceStrutHeight(self.0) }
    }

    pub fn set_force_strut_height(&mut self, force: bool) {
        unsafe { sys::skialin_bridge_StrutStyle_setForceStrutHeight(self.0, force) };
    }

    pub fn height_override(&self) -> bool {
        unsafe { sys::skialin_bridge_StrutStyle_getHeightOverride(self.0) }
    }

    pub fn set_height_override(&mut self, height_override: bool) {
        unsafe { sys::skialin_bridge_StrutStyle_setHeightOverride(self.0, height_override) };
    }

    pub fn half_leading(&self) -> bool {
        unsafe { sys::skialin_bridge_StrutStyle_getHalfLeading(self.0) }
    }

    pub fn set_half_leading(&mut self, half_leading: bool) {
        unsafe { sys::skialin_bridge_StrutStyle_setHalfLeading(self.0, half_leading) };
    }
}

impl Default for StrutStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for StrutStyle {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_StrutStyle_delete(self.0) };
    }
}
