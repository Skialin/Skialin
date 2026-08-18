use skialin_core::{color, Paint, Path, PathBuilder, PathDirection, PathEffect, Rect, Surface, TrimMode};

fn make_line_path() -> Path {
    let mut builder = PathBuilder::new();
    builder.add_rect(Rect::new(0.0, 0.0, 20.0, 20.0), PathDirection::Clockwise);
    builder.snapshot()
}

#[test]
fn dash_draws_without_crashing() {
    let effect = PathEffect::dash(&[4.0, 2.0], 0.0).unwrap();
    let mut paint = Paint::new();
    paint.set_style(skialin_core::PaintStyle::Stroke);
    paint.set_path_effect(Some(&effect));

    let mut surface = Surface::new_raster_n32_premul(32, 32).unwrap();
    let mut canvas = surface.canvas();
    canvas.draw_path(&make_line_path(), &paint);
}

#[test]
fn dash_none_for_odd_intervals() {
    assert!(PathEffect::dash(&[4.0, 2.0, 1.0], 0.0).is_none());
}

#[test]
fn corner_and_discrete_are_usable() {
    let corner = PathEffect::corner(3.0).unwrap();
    let discrete = PathEffect::discrete(5.0, 2.0, 0).unwrap();
    let mut paint = Paint::new();
    paint.set_path_effect(Some(&corner));
    paint.set_path_effect(Some(&discrete));
}

#[test]
fn trim_modes_are_usable() {
    let normal = PathEffect::trim(0.25, 0.75, TrimMode::Normal).unwrap();
    let inverted = PathEffect::trim(0.25, 0.75, TrimMode::Inverted).unwrap();
    let mut paint = Paint::new();
    paint.set_path_effect(Some(&normal));
    paint.set_path_effect(Some(&inverted));
}

#[test]
fn compose_and_sum() {
    let dash = PathEffect::dash(&[4.0, 2.0], 0.0).unwrap();
    let corner = PathEffect::corner(3.0).unwrap();
    let composed = PathEffect::compose(&dash, &corner);
    assert!(composed.is_some());
    let dash2 = PathEffect::dash(&[4.0, 2.0], 0.0).unwrap();
    let corner2 = PathEffect::corner(3.0).unwrap();
    let summed = PathEffect::sum(&dash2, &corner2);
    assert!(summed.is_some());
}

#[test]
fn set_path_effect_none_clears() {
    let effect = PathEffect::corner(3.0).unwrap();
    let mut paint = Paint::new();
    paint.set_path_effect(Some(&effect));
    paint.set_path_effect(None);
    assert!(paint.color_filter().is_none()); // sanity: paint still usable
    paint.set_color(color::RED);
}
