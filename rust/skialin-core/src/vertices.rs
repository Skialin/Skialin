use crate::{sys, Color, Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexMode {
    Triangles,
    TriangleStrip,
    TriangleFan,
}

impl From<VertexMode> for i32 {
    fn from(mode: VertexMode) -> Self {
        match mode {
            VertexMode::Triangles => 0,
            VertexMode::TriangleStrip => 1,
            VertexMode::TriangleFan => 2,
        }
    }
}

pub struct Vertices(pub(crate) *mut sys::SkVertices);

impl Vertices {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::SkVertices) -> Option<Self> {
        (!ptr.is_null()).then_some(Vertices(ptr))
    }

    /// `texs`/`colors`/`indices` are copied and may be empty.
    pub fn make_copy(mode: VertexMode, positions: &[Point], texs: &[Point], colors: &[Color], indices: &[u16]) -> Option<Self> {
        let sk_positions: Vec<sys::SkPoint> = positions.iter().map(|&p| p.into()).collect();
        let sk_texs: Vec<sys::SkPoint> = texs.iter().map(|&p| p.into()).collect();
        let texs_ptr = if sk_texs.is_empty() { std::ptr::null() } else { sk_texs.as_ptr() };
        let colors_ptr = if colors.is_empty() { std::ptr::null() } else { colors.as_ptr() };
        let indices_ptr = if indices.is_empty() { std::ptr::null() } else { indices.as_ptr() };
        unsafe {
            Self::from_raw(sys::skialin_bridge_Vertices_MakeCopy(
                mode.into(),
                sk_positions.len() as i32,
                sk_positions.as_ptr(),
                texs_ptr,
                colors_ptr,
                indices.len() as i32,
                indices_ptr,
            ))
        }
    }
}

impl Drop for Vertices {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Vertices_unref(self.0) };
    }
}
