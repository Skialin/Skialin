use crate::{sys, Font, Point, TextBlob};

pub struct TextBlobBuilder(*mut sys::SkTextBlobBuilder);

impl TextBlobBuilder {
    pub fn new() -> Self {
        TextBlobBuilder(unsafe { sys::skialin_bridge_TextBlobBuilder_new() })
    }

    pub fn build(&mut self) -> Option<TextBlob> {
        unsafe { TextBlob::from_raw(sys::skialin_bridge_TextBlobBuilder_make(self.0)) }
    }

    pub fn append_run(&mut self, font: &Font, glyphs: &[u16], x: f32, y: f32) {
        let buf = unsafe { sys::SkTextBlobBuilder_allocRun(self.0, font.0, glyphs.len() as i32, x, y, std::ptr::null()) };
        unsafe { std::ptr::copy_nonoverlapping(glyphs.as_ptr(), (*buf).glyphs, glyphs.len()) };
    }

    pub fn append_run_pos_h(&mut self, font: &Font, glyphs: &[u16], xpos: &[f32], y: f32) {
        let buf = unsafe { sys::SkTextBlobBuilder_allocRunPosH(self.0, font.0, glyphs.len() as i32, y, std::ptr::null()) };
        unsafe {
            std::ptr::copy_nonoverlapping(glyphs.as_ptr(), (*buf).glyphs, glyphs.len());
            std::ptr::copy_nonoverlapping(xpos.as_ptr(), (*buf).pos, xpos.len());
        }
    }

    pub fn append_run_pos(&mut self, font: &Font, glyphs: &[u16], pos: &[Point]) {
        let buf = unsafe { sys::SkTextBlobBuilder_allocRunPos(self.0, font.0, glyphs.len() as i32, std::ptr::null()) };
        let sk_points: Vec<sys::SkPoint> = pos.iter().map(|&p| p.into()).collect();
        unsafe {
            std::ptr::copy_nonoverlapping(glyphs.as_ptr(), (*buf).glyphs, glyphs.len());
            let points_ptr = (*buf).pos as *mut sys::SkPoint;
            std::ptr::copy_nonoverlapping(sk_points.as_ptr(), points_ptr, sk_points.len());
        }
    }

    pub fn append_run_rsxform(&mut self, font: &Font, glyphs: &[u16], xforms: &[f32]) {
        let buf = unsafe { sys::SkTextBlobBuilder_allocRunRSXform(self.0, font.0, glyphs.len() as i32) };
        let sk_xforms: Vec<sys::SkRSXform> = xforms
            .chunks_exact(4)
            .map(|c| sys::SkRSXform { fSCos: c[0], fSSin: c[1], fTx: c[2], fTy: c[3] })
            .collect();
        unsafe {
            std::ptr::copy_nonoverlapping(glyphs.as_ptr(), (*buf).glyphs, glyphs.len());
            let xforms_ptr = (*buf).pos as *mut sys::SkRSXform;
            std::ptr::copy_nonoverlapping(sk_xforms.as_ptr(), xforms_ptr, sk_xforms.len());
        }
    }
}

impl Default for TextBlobBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for TextBlobBuilder {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_TextBlobBuilder_delete(self.0) };
    }
}
