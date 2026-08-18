use skialin_core::{PathBuilder, PathDirection, PathMeasure, Rect};

fn make_square() -> skialin_core::Path {
    let mut builder = PathBuilder::new();
    builder.add_rect(Rect::new(0.0, 0.0, 10.0, 10.0), PathDirection::Clockwise);
    builder.snapshot()
}

#[test]
fn length_matches_square_perimeter() {
    let path = make_square();
    let mut measure = PathMeasure::new(&path, false, 1.0);
    assert_eq!(measure.length(), 40.0);
}

#[test]
fn pos_tan_at_zero_is_a_corner() {
    let path = make_square();
    let mut measure = PathMeasure::new(&path, false, 1.0);
    let pos_tan = measure.pos_tan(0.0).unwrap();
    assert_eq!(pos_tan.position.x, 0.0);
    assert_eq!(pos_tan.position.y, 0.0);
}

#[test]
fn matrix_at_distance_is_some() {
    let path = make_square();
    let mut measure = PathMeasure::new(&path, false, 1.0);
    assert!(measure.matrix(5.0).is_some());
}

#[test]
fn segment_appends_to_builder() {
    let path = make_square();
    let mut measure = PathMeasure::new(&path, false, 1.0);
    let mut dst = PathBuilder::new();
    let ok = measure.segment(0.0, 10.0, &mut dst, true);
    assert!(ok);
    let snapshot = dst.snapshot();
    assert!(!snapshot.is_empty());
}

#[test]
fn is_closed_and_next_contour() {
    let path = make_square();
    let mut measure = PathMeasure::new(&path, false, 1.0);
    assert!(measure.is_closed());
    assert!(!measure.next_contour());
}

#[test]
fn empty_measure_has_zero_length() {
    let mut measure = PathMeasure::empty();
    assert_eq!(measure.length(), 0.0);
    assert!(measure.pos_tan(0.0).is_none());
}

#[test]
fn set_path_updates_measure() {
    let path = make_square();
    let mut measure = PathMeasure::empty();
    measure.set_path(Some(&path), false);
    assert_eq!(measure.length(), 40.0);
    measure.set_path(None, false);
    assert_eq!(measure.length(), 0.0);
}
