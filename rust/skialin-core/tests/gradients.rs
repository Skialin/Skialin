use skialin_core::{color, Paint, Point, Shader, Surface, TileMode};

#[test]
fn linear_gradient_is_opaque_for_opaque_colors() {
    let shader = Shader::linear_gradient([Point::new(0.0, 0.0), Point::new(10.0, 0.0)], &[color::RED, color::BLUE], None, TileMode::Clamp, None).unwrap();
    assert!(shader.is_opaque());
}

#[test]
fn linear_gradient_with_positions() {
    let shader = Shader::linear_gradient(
        [Point::new(0.0, 0.0), Point::new(10.0, 0.0)],
        &[color::RED, color::GREEN, color::BLUE],
        Some(&[0.0, 0.25, 1.0]),
        TileMode::Clamp,
        None,
    );
    assert!(shader.is_some());
}

#[test]
fn radial_gradient_draws_without_crashing() {
    let shader = Shader::radial_gradient(Point::new(16.0, 16.0), 10.0, &[color::RED, color::BLUE], None, TileMode::Clamp, None).unwrap();
    let mut paint = Paint::new();
    paint.set_shader(Some(&shader));
    let mut surface = Surface::new_raster_n32_premul(32, 32).unwrap();
    let mut canvas = surface.canvas();
    canvas.draw_rect(skialin_core::Rect::new(0.0, 0.0, 32.0, 32.0), &paint);
}

#[test]
fn radial_gradient_none_for_empty_colors() {
    assert!(Shader::radial_gradient(Point::new(0.0, 0.0), 10.0, &[], None, TileMode::Clamp, None).is_none());
}

#[test]
fn two_point_conical_gradient_draws_without_crashing() {
    let shader = Shader::two_point_conical_gradient(Point::new(8.0, 8.0), 4.0, Point::new(16.0, 16.0), 12.0, &[color::RED, color::BLUE], None, TileMode::Clamp, None).unwrap();
    let mut paint = Paint::new();
    paint.set_shader(Some(&shader));
    let mut surface = Surface::new_raster_n32_premul(32, 32).unwrap();
    let mut canvas = surface.canvas();
    canvas.draw_rect(skialin_core::Rect::new(0.0, 0.0, 32.0, 32.0), &paint);
}

#[test]
fn sweep_gradient_draws_without_crashing() {
    let shader = Shader::sweep_gradient(Point::new(16.0, 16.0), 0.0, 360.0, &[color::RED, color::GREEN, color::BLUE], None, TileMode::Clamp, None).unwrap();
    let mut paint = Paint::new();
    paint.set_shader(Some(&shader));
    let mut surface = Surface::new_raster_n32_premul(32, 32).unwrap();
    let mut canvas = surface.canvas();
    canvas.draw_rect(skialin_core::Rect::new(0.0, 0.0, 32.0, 32.0), &paint);
}

#[test]
fn gradient_with_local_matrix() {
    let matrix = skialin_core::Matrix::translate(5.0, 5.0);
    let shader = Shader::linear_gradient([Point::new(0.0, 0.0), Point::new(10.0, 0.0)], &[color::RED, color::BLUE], None, TileMode::Clamp, Some(&matrix));
    assert!(shader.is_some());
}

#[test]
#[should_panic]
fn mismatched_positions_length_panics() {
    Shader::linear_gradient([Point::new(0.0, 0.0), Point::new(10.0, 0.0)], &[color::RED, color::BLUE], Some(&[0.0, 0.5, 1.0]), TileMode::Clamp, None);
}
