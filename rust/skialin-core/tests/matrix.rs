use skialin_core::{Matrix, Point, Rect};

#[test]
fn translate_maps_point() {
    let m = Matrix::translate(10.0, 20.0);
    let p = m.map_point(Point::new(1.0, 1.0));
    assert_eq!(p.x, 11.0);
    assert_eq!(p.y, 21.0);
}

#[test]
fn translate_maps_rect() {
    let m = Matrix::translate(10.0, 20.0);
    let r = m.map_rect(Rect::new(0.0, 0.0, 5.0, 5.0));
    assert_eq!(r, Rect::new(10.0, 20.0, 15.0, 25.0));
}
