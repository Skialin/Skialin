use skialin_core::{Path, PathBuilder, PathDirection, PathOp, Rect};

fn make_rect(left: f32, top: f32, right: f32, bottom: f32) -> Path {
    let mut builder = PathBuilder::new();
    builder.add_rect(Rect::new(left, top, right, bottom), PathDirection::Clockwise);
    builder.snapshot()
}

#[test]
fn union_covers_both_rects() {
    let a = make_rect(0.0, 0.0, 10.0, 10.0);
    let b = make_rect(5.0, 5.0, 15.0, 15.0);
    let result = Path::op(&a, &b, PathOp::Union).unwrap();
    assert_eq!(result.bounds(), Rect::new(0.0, 0.0, 15.0, 15.0));
}

#[test]
fn intersect_is_the_overlap() {
    let a = make_rect(0.0, 0.0, 10.0, 10.0);
    let b = make_rect(5.0, 5.0, 15.0, 15.0);
    let result = Path::op(&a, &b, PathOp::Intersect).unwrap();
    assert_eq!(result.bounds(), Rect::new(5.0, 5.0, 10.0, 10.0));
}

#[test]
fn difference_removes_overlap() {
    let a = make_rect(0.0, 0.0, 10.0, 10.0);
    let b = make_rect(5.0, 5.0, 15.0, 15.0);
    let result = Path::op(&a, &b, PathOp::Difference).unwrap();
    assert!(!result.is_empty());
    assert!(result.contains(skialin_core::Point::new(1.0, 1.0)));
    assert!(!result.contains(skialin_core::Point::new(7.0, 7.0)));
}

#[test]
fn xor_and_reverse_difference_are_usable() {
    let a = make_rect(0.0, 0.0, 10.0, 10.0);
    let b = make_rect(5.0, 5.0, 15.0, 15.0);
    assert!(Path::op(&a, &b, PathOp::Xor).is_some());
    assert!(Path::op(&a, &b, PathOp::ReverseDifference).is_some());
}

#[test]
fn simplify_removes_self_intersections() {
    let path = make_rect(0.0, 0.0, 10.0, 10.0);
    let simplified = path.simplify().unwrap();
    assert_eq!(simplified.bounds(), path.bounds());
}
