use skialin_core::{color, ColorFilter, Paint};

#[test]
fn reset_restores_defaults() {
    let mut paint = Paint::new();
    paint.set_color(color::RED);
    paint.reset();
    assert_eq!(paint.color(), color::BLACK);
}

#[test]
fn dither_roundtrips() {
    let mut paint = Paint::new();
    paint.set_dither(true);
    assert!(paint.is_dither());
}

#[test]
fn alpha_roundtrips() {
    let mut paint = Paint::new();
    paint.set_alpha(128);
    assert_eq!(paint.alpha(), 128);
    paint.set_alphaf(0.5);
    assert!((paint.alphaf() - 0.5).abs() < 0.01);
}

#[test]
fn set_argb_sets_color() {
    let mut paint = Paint::new();
    paint.set_argb(255, 10, 20, 30);
    assert_eq!(paint.color(), color::argb(255, 10, 20, 30));
}

#[test]
fn stroke_miter_roundtrips() {
    let mut paint = Paint::new();
    paint.set_stroke_miter(2.5);
    assert_eq!(paint.stroke_miter(), 2.5);
}

#[test]
fn nothing_to_draw_and_is_src_over() {
    let mut paint = Paint::new();
    assert!(!paint.nothing_to_draw());
    assert!(paint.is_src_over());
    paint.set_alpha(0);
    assert!(paint.nothing_to_draw());
}

#[test]
fn ref_getters_roundtrip() {
    let mut paint = Paint::new();
    assert!(paint.shader().is_none());
    let filter = ColorFilter::blend(color::RED, skialin_core::BlendMode::SrcOver).unwrap();
    paint.set_color_filter(Some(&filter));
    assert!(paint.color_filter().is_some());
}
