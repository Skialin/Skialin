use skialin_core::{color, Paint, Point, RRect, RRectType, Rect, Surface};

#[test]
fn make_rect_xy_has_simple_type() {
    let rrect = RRect::make_rect_xy(Rect::new(0.0, 0.0, 20.0, 20.0), 4.0, 4.0);
    assert_eq!(rrect.rrect_type(), RRectType::Simple);
    assert_eq!(rrect.rect(), Rect::new(0.0, 0.0, 20.0, 20.0));
}

#[test]
fn make_oval_has_oval_type() {
    let rrect = RRect::make_oval(Rect::new(0.0, 0.0, 20.0, 10.0));
    assert_eq!(rrect.rrect_type(), RRectType::Oval);
}

#[test]
fn make_rect_has_rect_type() {
    let rrect = RRect::make_rect(Rect::new(0.0, 0.0, 20.0, 20.0));
    assert_eq!(rrect.rrect_type(), RRectType::Rect);
    assert!(!rrect.is_empty());
}

#[test]
fn make_rect_radii_roundtrips_radii() {
    let radii = [(1.0, 2.0), (3.0, 4.0), (5.0, 6.0), (7.0, 8.0)];
    let rrect = RRect::make_rect_radii(Rect::new(0.0, 0.0, 40.0, 40.0), radii);
    assert_eq!(rrect.rrect_type(), RRectType::Complex);
    assert_eq!(rrect.radii(), radii);
}

#[test]
fn contains_point_and_rect() {
    let rrect = RRect::make_rect(Rect::new(0.0, 0.0, 20.0, 20.0));
    assert!(rrect.contains_point(Point::new(10.0, 10.0)));
    assert!(!rrect.contains_point(Point::new(100.0, 100.0)));
    assert!(rrect.contains_rect(Rect::new(2.0, 2.0, 18.0, 18.0)));
    assert!(rrect.is_valid());
}

#[test]
fn inset_and_outset() {
    let rrect = RRect::make_rect(Rect::new(0.0, 0.0, 20.0, 20.0));
    let inset = rrect.inset(2.0, 2.0);
    assert_eq!(inset.rect(), Rect::new(2.0, 2.0, 18.0, 18.0));
    let outset = rrect.outset(2.0, 2.0);
    assert_eq!(outset.rect(), Rect::new(-2.0, -2.0, 22.0, 22.0));
}

#[test]
fn clone_is_independent() {
    let rrect = RRect::make_rect(Rect::new(0.0, 0.0, 20.0, 20.0));
    let cloned = rrect.clone();
    assert_eq!(cloned.rect(), rrect.rect());
}

#[test]
fn draws_and_clips_without_crashing() {
    let outer = RRect::make_rect_xy(Rect::new(0.0, 0.0, 32.0, 32.0), 4.0, 4.0);
    let inner = RRect::make_rect_xy(Rect::new(4.0, 4.0, 28.0, 28.0), 2.0, 2.0);
    let mut surface = Surface::new_raster_n32_premul(32, 32).unwrap();
    let mut canvas = surface.canvas;
    let mut paint = Paint::new();
    paint.set_color(color::RED);
    canvas.draw_rrect(&outer, &paint);
    canvas.draw_drrect(&outer, &inner, &paint);
    canvas.clip_rrect(&outer, skialin_core::ClipOp::Intersect, false);
}
