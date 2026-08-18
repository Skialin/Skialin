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
}

impl Drop for TextBlob {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_TextBlob_unref(self.0) };
    }
}
