use skialin_core::{color, Paint, Point, Surface, VertexMode, Vertices, M44};

#[test]
fn identity_maps_a_point_to_itself() {
    let m = M44::identity();
    let out = m.map([1.0, 2.0, 3.0, 1.0]);
    assert_eq!(out, [1.0, 2.0, 3.0, 1.0]);
}

#[test]
fn translate_shifts_a_point() {
    let m = M44::translate(5.0, 6.0, 0.0);
    let out = m.map([1.0, 2.0, 0.0, 1.0]);
    assert_eq!(out, [6.0, 8.0, 0.0, 1.0]);
}

#[test]
fn scale_scales_a_point() {
    let m = M44::scale(2.0, 3.0, 1.0);
    let out = m.map([1.0, 1.0, 1.0, 1.0]);
    assert_eq!(out, [2.0, 3.0, 1.0, 1.0]);
}

#[test]
fn concat_composes_transforms() {
    let translate = M44::translate(10.0, 0.0, 0.0);
    let scale = M44::scale(2.0, 2.0, 1.0);
    let combined = M44::concat(&translate, &scale);
    let out = combined.map([1.0, 1.0, 0.0, 1.0]);
    assert_eq!(out, [12.0, 2.0, 0.0, 1.0]);
}

#[test]
fn invert_undoes_translate() {
    let m = M44::translate(3.0, 4.0, 0.0);
    let inv = m.invert().unwrap();
    let out = inv.map([3.0, 4.0, 0.0, 1.0]);
    assert!((out[0]).abs() < 1e-5);
    assert!((out[1]).abs() < 1e-5);
}

#[test]
fn row_major_roundtrips() {
    let mut values = [0f32; 16];
    for (i, v) in values.iter_mut().enumerate() {
        *v = i as f32;
    }
    // Make it a valid-looking matrix (identity-like diagonal survives roundtrip regardless).
    let m = M44::from_row_major(&values);
    assert_eq!(m.to_row_major(), values);
}

#[test]
fn equality_and_clone() {
    let a = M44::translate(1.0, 2.0, 3.0);
    let b = a.clone();
    assert!(a == b);
    let c = M44::identity();
    assert!(a != c);
}

#[test]
fn vertices_draws_a_triangle_without_crashing() {
    let positions = [Point::new(0.0, 0.0), Point::new(10.0, 0.0), Point::new(5.0, 10.0)];
    let colors = [color::RED, color::GREEN, color::BLUE];
    let vertices = Vertices::make_copy(VertexMode::Triangles, &positions, &[], &colors, &[]).unwrap();

    let mut surface = Surface::new_raster_n32_premul(16, 16).unwrap();
    let mut canvas = surface.canvas;
    let paint = Paint::new();
    canvas.draw_vertices(&vertices, skialin_core::BlendMode::SrcOver, &paint);
}

#[test]
fn vertices_with_indices_and_texs() {
    let positions = [Point::new(0.0, 0.0), Point::new(10.0, 0.0), Point::new(10.0, 10.0), Point::new(0.0, 10.0)];
    let texs = positions;
    let indices = [0u16, 1, 2, 0, 2, 3];
    let vertices = Vertices::make_copy(VertexMode::Triangles, &positions, &texs, &[], &indices);
    assert!(vertices.is_some());
}

#[test]
fn concat_44_draws_without_crashing() {
    let mut surface = Surface::new_raster_n32_premul(16, 16).unwrap();
    let mut canvas = surface.canvas;
    canvas.concat_44(&M44::translate(2.0, 2.0, 0.0));
    let mut paint = Paint::new();
    paint.set_color(color::RED);
    canvas.draw_rect(skialin_core::Rect::new(0.0, 0.0, 4.0, 4.0), &paint);
}
