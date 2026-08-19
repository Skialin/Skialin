use crate::{sys, IRect, Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionOp {
    Difference,
    Intersect,
    Union,
    Xor,
    ReverseDifference,
    Replace,
}

impl From<RegionOp> for i32 {
    fn from(op: RegionOp) -> Self {
        match op {
            RegionOp::Difference => 0,
            RegionOp::Intersect => 1,
            RegionOp::Union => 2,
            RegionOp::Xor => 3,
            RegionOp::ReverseDifference => 4,
            RegionOp::Replace => 5,
        }
    }
}

pub struct Region(pub(crate) *mut sys::SkRegion);

impl Region {
    pub fn new() -> Self {
        Region(unsafe { sys::skialin_bridge_Region_MakeEmpty() })
    }

    pub fn from_rect(rect: IRect) -> Self {
        let sk_rect: sys::SkIRect = rect.into();
        Region(unsafe { sys::skialin_bridge_Region_MakeRect(&sk_rect) })
    }

    pub fn set_rect(&mut self, rect: IRect) -> bool {
        let sk_rect: sys::SkIRect = rect.into();
        unsafe { sys::skialin_bridge_Region_setRect(self.0, &sk_rect) }
    }

    pub fn set_path(&mut self, path: &Path, clip: &Region) -> bool {
        unsafe { sys::skialin_bridge_Region_setPath(self.0, path.0, clip.0) }
    }

    pub fn op(&mut self, other: &Region, op: RegionOp) -> bool {
        unsafe { sys::skialin_bridge_Region_opRegion(self.0, other.0, op.into()) }
    }

    pub fn op_rect(&mut self, rect: IRect, op: RegionOp) -> bool {
        let sk_rect: sys::SkIRect = rect.into();
        unsafe { sys::skialin_bridge_Region_opRect(self.0, &sk_rect, op.into()) }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { sys::skialin_bridge_Region_isEmpty(self.0) }
    }

    pub fn is_rect(&self) -> bool {
        unsafe { sys::skialin_bridge_Region_isRect(self.0) }
    }

    pub fn is_complex(&self) -> bool {
        unsafe { sys::skialin_bridge_Region_isComplex(self.0) }
    }

    pub fn bounds(&self) -> IRect {
        let mut out = sys::SkIRect::default();
        unsafe { sys::skialin_bridge_Region_getBounds(self.0, &mut out) };
        out.into()
    }

    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        unsafe { sys::skialin_bridge_Region_containsPoint(self.0, x, y) }
    }

    pub fn contains_rect(&self, rect: IRect) -> bool {
        let sk_rect: sys::SkIRect = rect.into();
        unsafe { sys::skialin_bridge_Region_containsRect(self.0, &sk_rect) }
    }

    pub fn contains_region(&self, other: &Region) -> bool {
        unsafe { sys::skialin_bridge_Region_containsRegion(self.0, other.0) }
    }

    pub fn intersects_rect(&self, rect: IRect) -> bool {
        let sk_rect: sys::SkIRect = rect.into();
        unsafe { sys::skialin_bridge_Region_intersectsRect(self.0, &sk_rect) }
    }

    pub fn intersects_region(&self, other: &Region) -> bool {
        unsafe { sys::skialin_bridge_Region_intersectsRegion(self.0, other.0) }
    }

    pub fn boundary_path(&self) -> Path {
        unsafe { Path::from_raw(sys::skialin_bridge_Region_getBoundaryPath(self.0)) }.expect("getBoundaryPath never returns null")
    }
}

impl Default for Region {
    fn default() -> Self {
        Region::new()
    }
}

impl Clone for Region {
    fn clone(&self) -> Self {
        Region(unsafe { sys::skialin_bridge_Region_clone(self.0) })
    }
}

impl PartialEq for Region {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sys::skialin_bridge_Region_equals(self.0, other.0) }
    }
}

impl Drop for Region {
    fn drop(&mut self) {
        unsafe { sys::skialin_bridge_Region_delete(self.0) };
    }
}
