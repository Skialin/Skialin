use skialin_core::{Paint, RuntimeEffect, Surface};

#[test]
fn shader_effect_compiles_and_draws() {
    let sksl = r#"
        vec4 main(vec2 coord) {
            return vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;
    let effect = RuntimeEffect::make_for_shader(sksl).unwrap();
    let shader = effect.make_shader(&[], &[], None).unwrap();

    let mut paint = Paint::new();
    paint.set_shader(Some(&shader));
    let mut surface = Surface::new_raster_n32_premul(8, 8).unwrap();
    let mut canvas = surface.canvas;
    canvas.draw_rect(skialin_core::Rect::new(0.0, 0.0, 8.0, 8.0), &paint);
}

#[test]
fn shader_effect_with_uniform() {
    let sksl = r#"
        uniform half4 color;
        vec4 main(vec2 coord) {
            return color;
        }
    "#;
    let effect = RuntimeEffect::make_for_shader(sksl).unwrap();
    let uniform_bytes: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    let bytes: &[u8] = bytemuck_cast(&uniform_bytes);
    let shader = effect.make_shader(bytes, &[], None).unwrap();

    let mut paint = Paint::new();
    paint.set_shader(Some(&shader));
    let mut surface = Surface::new_raster_n32_premul(8, 8).unwrap();
    let mut canvas = surface.canvas;
    canvas.draw_rect(skialin_core::Rect::new(0.0, 0.0, 8.0, 8.0), &paint);
}

fn bytemuck_cast(values: &[f32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(values.as_ptr().cast(), std::mem::size_of_val(values)) }
}

#[test]
fn color_filter_effect_compiles() {
    let sksl = r#"
        vec4 main(vec4 inColor) {
            return inColor.bgra;
        }
    "#;
    let effect = RuntimeEffect::make_for_color_filter(sksl).unwrap();
    let filter = effect.make_color_filter(&[], &[]).unwrap();

    let mut paint = Paint::new();
    paint.set_color_filter(Some(&filter));
}

#[test]
fn invalid_sksl_returns_error() {
    match RuntimeEffect::make_for_shader("this is not valid sksl") {
        Err(message) => assert!(!message.is_empty()),
        Ok(_) => panic!("expected invalid SkSL to fail to compile"),
    }
}

#[test]
fn shader_effect_with_child_shader() {
    let sksl = r#"
        uniform shader child;
        vec4 main(vec2 coord) {
            return child.eval(coord);
        }
    "#;
    let effect = RuntimeEffect::make_for_shader(sksl).unwrap();
    let child = skialin_core::Shader::color(skialin_core::color::RED);
    let shader = effect.make_shader(&[], &[&child], None).unwrap();

    let mut paint = Paint::new();
    paint.set_shader(Some(&shader));
    let mut surface = Surface::new_raster_n32_premul(8, 8).unwrap();
    let mut canvas = surface.canvas;
    canvas.draw_rect(skialin_core::Rect::new(0.0, 0.0, 8.0, 8.0), &paint);
}
