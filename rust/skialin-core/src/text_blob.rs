use crate::{sys, Font, Rect, TextEncoding};

pub struct TextBlob(pub(crate) *mut sys::SkTextBlob);

impl TextBlob {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::SkTextBlob) -> Option<Self> {
        (!ptr.is_null()).then_some(TextBlob(ptr))
    }

    /// A single run of `text`, positioned using `font`'s default advances.
    /// `None` if `text` is empty.
    pub fn from_text(text: &str, font: &Font, encoding: TextEncoding) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_TextBlob_MakeFromText(text.as_ptr().cast(), text.len(), font.0, encoding.into())) }
    }

    /// A single run of `text`, positioned at `xpos[i]` on the shared
    /// baseline `const_y`. `xpos.len()` must equal the glyph/character
    /// count implied by `text` and `encoding`. `None` if `text` is empty.
    pub fn from_pos_text_h(text: &str, xpos: &[f32], const_y: f32, font: &Font, encoding: TextEncoding) -> Option<Self> {
        unsafe { Self::from_raw(sys::skialin_bridge_TextBlob_MakeFromPosTextH(text.as_ptr().cast(), text.len(), xpos.as_ptr(), xpos.len(), const_y, font.0, encoding.into())) }
    }

    /// A single run of `text`, positioned at `pos[i]`. `pos.len()` must
    /// equal the glyph/character count implied by `text` and `encoding`.
    /// `None` if `text` is empty.
    pub fn from_pos_text(text: &str, pos: &[crate::Point], font: &Font, encoding: TextEncoding) -> Option<Self> {
        let pos: Vec<sys::SkPoint> = pos.iter().map(|&p| p.into()).collect();
        unsafe { Self::from_raw(sys::skialin_bridge_TextBlob_MakeFromPosText(text.as_ptr().cast(), text.len(), pos.as_ptr(), pos.len(), font.0, encoding.into())) }
    }

    pub fn bounds(&self) -> Rect {
        let bounds = unsafe { sys::SkTextBlob_bounds(self.0) };
        unsafe { *bounds }.into()
    }

    pub fn unique_id(&self) -> u32 {
        unsafe { sys::SkTextBlob_uniqueID(self.0) }
    }

    pub fn from_rsxform(text: &str, xforms: &[f32], font: &Font, encoding: TextEncoding) -> Option<Self> {
        let sk_xforms: Vec<sys::SkRSXform> = xforms.chunks_exact(4).map(|c| sys::SkRSXform { fSCos: c[0], fSSin: c[1], fTx: c[2], fTy: c[3] }).collect();
        unsafe { Self::from_raw(sys::skialin_bridge_TextBlob_MakeFromRSXform(text.as_ptr().cast(), text.len(), sk_xforms.as_ptr(), sk_xforms.len(), font.0, encoding.into())) }
    }

    pub fn get_intercepts(&self, lower: f32, upper: f32, paint: Option<&crate::Paint>) -> Vec<f32> {
        let bounds = [lower, upper];
        let paint_ptr = paint.map_or(std::ptr::null(), |p| &*p.0 as *const sys::SkPaint);
        let count = unsafe { sys::SkTextBlob_getIntercepts(self.0, bounds.as_ptr(), std::ptr::null_mut(), paint_ptr) };
        let mut intervals = vec![0f32; count as usize];
        unsafe { sys::SkTextBlob_getIntercepts(self.0, bounds.as_ptr(), intervals.as_mut_ptr(), paint_ptr) };
        intervals
    }

    pub fn serialize_to_data(&self) -> crate::Data {
        unsafe { crate::Data::from_raw(sys::skialin_bridge_TextBlob_serialize(self.0)) }.expect("serialize never returns null")
    }

    pub fn from_data(data: &crate::Data) -> Option<Self> {
        let bytes = data.as_bytes();
        unsafe { Self::from_raw(sys::skialin_bridge_TextBlob_Deserialize(bytes.as_ptr().cast(), bytes.len())) }
    }
}

impl Drop for TextBlob {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_TextBlob_unref(self.0) };
    }
}
