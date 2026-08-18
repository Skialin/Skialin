use skialin_core::{color, Image, ImageInfo, Paint, Point, PointMode, Rect, SamplingOptions, SrcRectConstraint, Surface};

fn make_image() -> Image {
    let info = ImageInfo::n32_premul(4, 4);
    let pixels = vec![0xFFu8; 4 * 4 * 4];
    let data = skialin_core::Data::with_copy(&pixels);
    Image::from_data(&info, &data, 16).unwrap()
}

#[test]
fn skew_and_matrix_roundtrip() {
    let mut surface = Surface::new_raster_n32_premul(16, 16).unwrap();
    let mut canvas = surface.canvas();
    canvas.skew(0.1, 0.0);
    let matrix = canvas.total_matrix();
    canvas.set_matrix(&matrix);
    canvas.reset_matrix();
    let identity = canvas.total_matrix();
    assert_eq!(identity.map_point(Point::new(1.0, 0.0)).x, 1.0);
}

#[test]
fn quick_reject_rect_outside_clip() {
    let mut surface = Surface::new_raster_n32_premul(16, 16).unwrap();
    let mut canvas = surface.canvas();
    canvas.clip_rect(Rect::new(0.0, 0.0, 8.0, 8.0), skialin_core::ClipOp::Intersect);
    assert!(canvas.quick_reject_rect(Rect::new(100.0, 100.0, 200.0, 200.0)));
    assert!(!canvas.quick_reject_rect(Rect::new(0.0, 0.0, 4.0, 4.0)));
}

#[test]
fn draw_round_rect_and_arc() {
    let mut surface = Surface::new_raster_n32_premul(16, 16).unwrap();
    let mut canvas = surface.canvas();
    let mut paint = Paint::new();
    paint.set_color(color::RED);
    canvas.draw_round_rect(Rect::new(0.0, 0.0, 16.0, 16.0), 2.0, 2.0, &paint);
    canvas.draw_arc(Rect::new(0.0, 0.0, 16.0, 16.0), 0.0, 180.0, true, &paint);
}

#[test]
fn draw_points_variants() {
    let mut surface = Surface::new_raster_n32_premul(16, 16).unwrap();
    let mut canvas = surface.canvas();
    let mut paint = Paint::new();
    paint.set_color(color::BLUE);
    let pts = [Point::new(1.0, 1.0), Point::new(5.0, 5.0), Point::new(10.0, 2.0)];
    canvas.draw_points(PointMode::Points, &pts, &paint);
    canvas.draw_points(PointMode::Lines, &pts, &paint);
    canvas.draw_points(PointMode::Polygon, &pts, &paint);
}

#[test]
fn draw_image_and_image_rect() {
    let image = make_image();
    let mut surface = Surface::new_raster_n32_premul(16, 16).unwrap();
    let mut canvas = surface.canvas();
    canvas.draw_image(&image, 0.0, 0.0, SamplingOptions::default(), None);

    let mut paint = Paint::new();
    paint.set_anti_alias(true);
    canvas.draw_image_rect(&image, None, Rect::new(0.0, 0.0, 16.0, 16.0), SamplingOptions::default(), Some(&paint), SrcRectConstraint::Fast);
    canvas.draw_image_rect(&image, Some(Rect::new(0.0, 0.0, 2.0, 2.0)), Rect::new(0.0, 0.0, 8.0, 8.0), SamplingOptions::default(), Some(&paint), SrcRectConstraint::Strict);
}

#[test]
fn save_layer_returns_incrementing_count() {
    let mut surface = Surface::new_raster_n32_premul(16, 16).unwrap();
    let mut canvas = surface.canvas();
    let count_before = canvas.save();
    let layer_count = canvas.save_layer(Some(Rect::new(0.0, 0.0, 16.0, 16.0)), None);
    assert!(layer_count > count_before);
    canvas.restore();
}
