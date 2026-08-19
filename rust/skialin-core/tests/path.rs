use skialin_core::{PathBuilder, Point, Rect};

#[test]
fn triangle_contains_interior_point() {
    let mut builder = PathBuilder::new();
    builder.move_to(Point::new(0.0, 0.0)).line_to(Point::new(10.0, 0.0)).line_to(Point::new(10.0, 10.0)).close();
    assert!(!builder.is_empty());

    let path = builder.snapshot();
    assert!(!path.is_empty());
    assert!(path.contains(Point::new(7.0, 3.0)));
    assert!(!path.contains(Point::new(1.0, 50.0)));
}

#[test]
fn rect_points_and_bounds_match_known_shape() {
    let mut builder = PathBuilder::new();
    builder.add_rect(Rect::new(0.0, 0.0, 10.0, 20.0), skialin_core::PathDirection::Clockwise);
    let path = builder.snapshot();

    assert_eq!(path.points_count(), 4);
    let pts = path.points();
    assert_eq!(pts.len(), 4);
    assert_eq!((pts[0].x, pts[0].y), (0.0, 0.0));

    let b = path.compute_tight_bounds();
    assert_eq!((b.left, b.top, b.right, b.bottom), (0.0, 0.0, 10.0, 20.0));

    assert!(path.is_convex());
}
