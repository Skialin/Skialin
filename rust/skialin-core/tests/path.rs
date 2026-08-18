use skialin_core::{PathBuilder, Point};

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
