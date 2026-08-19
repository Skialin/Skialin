use skialin_core::{color, BlendMode, BlurStyle, ColorFilter, ImageFilter, MaskFilter, Paint, Path, PathBuilder, PathDirection, Surface, TileMode};

#[test]
fn color_filter_blend_is_usable() {
    let filter = ColorFilter::blend(color::RED, BlendMode::SrcOver).unwrap();
    let mut paint = Paint::new();
    paint.set_color_filter(Some(&filter));
    paint.set_color_filter(None);
}

#[test]
fn color_filter_matrix_is_usable() {
    let identity: [f32; 20] = [
        1.0, 0.0, 0.0, 0.0, 0.0, //
        0.0, 1.0, 0.0, 0.0, 0.0, //
        0.0, 0.0, 1.0, 0.0, 0.0, //
        0.0, 0.0, 0.0, 1.0, 0.0,
    ];
    let filter = ColorFilter::matrix(&identity, true).unwrap();
    let mut paint = Paint::new();
    paint.set_color_filter(Some(&filter));
}

#[test]
fn color_filter_compose_and_lerp() {
    let a = ColorFilter::blend(color::RED, BlendMode::SrcOver).unwrap();
    let b = ColorFilter::blend(color::BLUE, BlendMode::SrcOver).unwrap();
    let composed = ColorFilter::compose(&a, &b);
    assert!(composed.is_some());
    let lerped = ColorFilter::lerp(0.5, &a, &b);
    assert!(lerped.is_some());
}

#[test]
fn image_filter_blur_draws_without_crashing() {
    let filter = ImageFilter::blur(4.0, 4.0, TileMode::Decal, None).unwrap();
    let mut paint = Paint::new();
    paint.set_image_filter(Some(&filter));

    let mut surface = Surface::new_raster_n32_premul(32, 32).unwrap();
    let mut canvas = surface.canvas;
    let mut builder = PathBuilder::new();
    builder.add_rect(skialin_core::Rect::new(4.0, 4.0, 20.0, 20.0), PathDirection::Clockwise);
    let path: Path = builder.snapshot();
    canvas.draw_path(&path, &paint);
}

#[test]
fn image_filter_compose_chain() {
    let blur = ImageFilter::blur(2.0, 2.0, TileMode::Decal, None).unwrap();
    let offset = ImageFilter::offset(3.0, 3.0, Some(&blur)).unwrap();
    let composed = ImageFilter::compose(&offset, &blur);
    assert!(composed.is_some());
}

#[test]
fn image_filter_drop_shadow_and_morphology() {
    assert!(ImageFilter::drop_shadow(2.0, 2.0, 3.0, 3.0, color::BLACK, None).is_some());
    assert!(ImageFilter::drop_shadow_only(2.0, 2.0, 3.0, 3.0, color::BLACK, None).is_some());
    assert!(ImageFilter::dilate(2.0, 2.0, None).is_some());
    assert!(ImageFilter::erode(2.0, 2.0, None).is_some());
}

#[test]
fn image_filter_matrix_transform() {
    let matrix = skialin_core::Matrix::translate(5.0, 5.0);
    let filter = ImageFilter::matrix_transform(&matrix, skialin_core::SamplingOptions::default(), None);
    assert!(filter.is_some());
}

#[test]
fn mask_filter_blur_draws_without_crashing() {
    let filter = MaskFilter::blur(BlurStyle::Normal, 3.0, true).unwrap();
    let mut paint = Paint::new();
    paint.set_mask_filter(Some(&filter));

    let mut surface = Surface::new_raster_n32_premul(32, 32).unwrap();
    let mut canvas = surface.canvas;
    canvas.draw_circle(skialin_core::Point::new(16.0, 16.0), 8.0, &paint);
}
