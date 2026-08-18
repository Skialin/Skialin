use skialin_core::{Matrix, Paint, Shader};

#[test]
fn color_shader_is_opaque_for_opaque_color() {
    let shader = Shader::color(0xFFFF0000);
    assert!(shader.is_opaque());

    let translucent = Shader::color(0x80FF0000);
    assert!(!translucent.is_opaque());
}

#[test]
fn with_local_matrix_produces_a_new_shader() {
    let shader = Shader::color(0xFF00FF00);
    let moved = shader.with_local_matrix(&Matrix::translate(10.0, 10.0));
    assert!(moved.is_opaque());
}

#[test]
fn attaches_to_paint() {
    let mut paint = Paint::new();
    paint.set_shader(Some(&Shader::color(0xFF0000FF)));
    paint.set_shader(None);
}
