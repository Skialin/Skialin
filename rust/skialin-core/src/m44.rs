use crate::sys;

pub struct M44(pub(crate) *mut sys::SkM44);

impl M44 {
    pub(crate) unsafe fn from_raw(ptr: *mut sys::SkM44) -> Option<Self> {
        (!ptr.is_null()).then_some(M44(ptr))
    }

    pub fn identity() -> Self {
        M44(unsafe { sys::skialin_bridge_M44_MakeIdentity() })
    }

    /// `row_major` is 16 floats, row by row.
    pub fn from_row_major(row_major: &[f32; 16]) -> Self {
        M44(unsafe { sys::skialin_bridge_M44_MakeFromRowMajor(row_major.as_ptr()) })
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        M44(unsafe { sys::skialin_bridge_M44_MakeTranslate(x, y, z) })
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        M44(unsafe { sys::skialin_bridge_M44_MakeScale(x, y, z) })
    }

    /// `axis` need not be normalized.
    pub fn rotate(axis: (f32, f32, f32), radians: f32) -> Self {
        M44(unsafe { sys::skialin_bridge_M44_MakeRotate(axis.0, axis.1, axis.2, radians) })
    }

    /// 16 floats, row by row.
    pub fn to_row_major(&self) -> [f32; 16] {
        let mut out = [0f32; 16];
        unsafe { sys::skialin_bridge_M44_getRowMajor(self.0, out.as_mut_ptr()) };
        out
    }

    pub fn transpose(&self) -> Self {
        M44(unsafe { sys::skialin_bridge_M44_transpose(self.0) })
    }

    pub fn rc(&self, row: i32, col: i32) -> f32 {
        unsafe { sys::skialin_bridge_M44_rc(self.0, row, col) }
    }

    pub fn concat(a: &M44, b: &M44) -> Self {
        M44(unsafe { sys::skialin_bridge_M44_concat(a.0, b.0) })
    }

    /// `None` if this matrix isn't invertible.
    pub fn invert(&self) -> Option<M44> {
        unsafe { Self::from_raw(sys::skialin_bridge_M44_invert(self.0)) }
    }

    /// Transforms the 4-component vector `[x, y, z, w]`.
    pub fn map(&self, v: [f32; 4]) -> [f32; 4] {
        let mut out = [0f32; 4];
        unsafe { sys::skialin_bridge_M44_mapV4(self.0, v.as_ptr(), out.as_mut_ptr()) };
        out
    }
}

impl Clone for M44 {
    fn clone(&self) -> Self {
        M44(unsafe { sys::skialin_bridge_M44_clone(self.0) })
    }
}

impl PartialEq for M44 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sys::skialin_bridge_M44_equals(self.0, other.0) }
    }
}

impl Drop for M44 {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_M44_delete(self.0) };
    }
}
