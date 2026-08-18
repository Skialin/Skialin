use crate::sys;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
}

impl From<Point> for sys::SkPoint {
    fn from(p: Point) -> Self {
        sys::SkPoint { fX: p.x, fY: p.y }
    }
}

impl From<sys::SkPoint> for Point {
    fn from(p: sys::SkPoint) -> Self {
        Point { x: p.fX, y: p.fY }
    }
}
