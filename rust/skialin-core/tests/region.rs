use skialin_core::{color, IRect, Paint, Region, RegionOp, Surface};

#[test]
fn from_rect_reports_bounds_and_type() {
    let region = Region::from_rect(IRect::new(0, 0, 10, 20));
    assert!(region.is_rect());
    assert!(!region.is_empty());
    assert!(!region.is_complex());
    assert_eq!(region.bounds(), IRect::new(0, 0, 10, 20));
}

#[test]
fn union_produces_complex_region() {
    let mut region = Region::from_rect(IRect::new(0, 0, 10, 10));
    let other = Region::from_rect(IRect::new(20, 20, 30, 30));
    assert!(region.op(&other, RegionOp::Union));
    assert!(region.is_complex());
    assert_eq!(region.bounds(), IRect::new(0, 0, 30, 30));
}

#[test]
fn contains_and_intersects() {
    let region = Region::from_rect(IRect::new(0, 0, 10, 10));
    assert!(region.contains_point(5, 5));
    assert!(!region.contains_point(15, 15));
    assert!(region.intersects_rect(IRect::new(5, 5, 15, 15)));
    assert!(!region.intersects_rect(IRect::new(20, 20, 30, 30)));
}

#[test]
fn clone_is_independent() {
    let mut region = Region::from_rect(IRect::new(0, 0, 10, 10));
    let clone = region.clone();
    region.set_rect(IRect::new(0, 0, 5, 5));
    assert_eq!(clone.bounds(), IRect::new(0, 0, 10, 10));
    assert_eq!(region.bounds(), IRect::new(0, 0, 5, 5));
}

#[test]
fn draws_and_clips_without_crashing() {
    let region = Region::from_rect(IRect::new(0, 0, 16, 16));
    let mut surface = Surface::new_raster_n32_premul(16, 16).unwrap();
    let mut canvas = surface.canvas;
    let mut paint = Paint::new();
    paint.set_color(color::RED);
    canvas.draw_region(&region, &paint);
    canvas.clip_region(&region, skialin_core::ClipOp::Intersect);
}
