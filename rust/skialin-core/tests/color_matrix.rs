use skialin_core::{ColorFilter, ColorMatrix, Paint};

#[test]
fn identity_is_the_default_20_floats() {
    let m = ColorMatrix::identity();
    let expected: [f32; 20] = [
        1.0, 0.0, 0.0, 0.0, 0.0, //
        0.0, 1.0, 0.0, 0.0, 0.0, //
        0.0, 0.0, 1.0, 0.0, 0.0, //
        0.0, 0.0, 0.0, 1.0, 0.0,
    ];
    assert_eq!(m.0, expected);
}

#[test]
fn saturation_zero_is_usable_in_a_filter() {
    let m = ColorMatrix::saturation(0.0);
    let filter = ColorFilter::matrix(&m.0, true).unwrap();
    let mut paint = Paint::new();
    paint.set_color_filter(Some(&filter));
}

#[test]
fn scale_sets_diagonal() {
    let m = ColorMatrix::scale(0.5, 0.6, 0.7, 1.0);
    assert_eq!(m.0[0], 0.5);
    assert_eq!(m.0[6], 0.6);
    assert_eq!(m.0[12], 0.7);
    assert_eq!(m.0[18], 1.0);
}

#[test]
fn post_translate_sets_last_column() {
    let mut m = ColorMatrix::identity();
    m.post_translate(0.1, 0.2, 0.3, 0.0);
    assert_eq!(m.0[4], 0.1);
    assert_eq!(m.0[9], 0.2);
    assert_eq!(m.0[14], 0.3);
}

#[test]
fn concat_is_usable() {
    let a = ColorMatrix::scale(0.5, 0.5, 0.5, 1.0);
    let b = ColorMatrix::saturation(0.0);
    let combined = ColorMatrix::concat(&a, &b);
    let filter = ColorFilter::matrix(&combined.0, true);
    assert!(filter.is_some());
}
