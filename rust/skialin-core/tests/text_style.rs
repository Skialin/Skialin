use skialin_core::{color, Decoration, FontStyle, TextDecoration, TextDecorationMode, TextDecorationStyle, TextStyle};

#[test]
fn default_font_size_is_14() {
    let style = TextStyle::new();
    assert_eq!(style.font_size(), 14.0);
}

#[test]
fn color_roundtrips() {
    let mut style = TextStyle::new();
    style.set_color(color::RED);
    assert_eq!(style.color(), color::RED);
}

#[test]
fn font_families_roundtrip() {
    let mut style = TextStyle::new();
    style.set_font_families(&["Arial", "Helvetica"]);
    assert_eq!(style.font_families(), vec!["Arial".to_string(), "Helvetica".to_string()]);
}

#[test]
fn font_style_roundtrips() {
    let mut style = TextStyle::new();
    style.set_font_style(FontStyle::bold());
    assert_eq!(style.font_style(), FontStyle::bold());
}

#[test]
fn decoration_roundtrips() {
    let mut style = TextStyle::new();
    style.set_decoration(TextDecoration::UNDERLINE | TextDecoration::LINE_THROUGH);
    style.set_decoration_mode(TextDecorationMode::Gaps);
    style.set_decoration_color(color::BLUE);
    style.set_decoration_style(TextDecorationStyle::Dashed);
    style.set_decoration_thickness_multiplier(2.0);

    let decoration: Decoration = style.decoration();
    assert_eq!(decoration.decoration, TextDecoration::UNDERLINE | TextDecoration::LINE_THROUGH);
    assert_eq!(decoration.mode, TextDecorationMode::Gaps);
    assert_eq!(decoration.color, color::BLUE);
    assert_eq!(decoration.style, TextDecorationStyle::Dashed);
    assert_eq!(decoration.thickness_multiplier, 2.0);
}

#[test]
fn spacing_and_height_roundtrip() {
    let mut style = TextStyle::new();
    style.set_letter_spacing(1.5);
    style.set_word_spacing(3.0);
    style.set_height_override(true);
    style.set_height(2.0);
    assert_eq!(style.letter_spacing(), 1.5);
    assert_eq!(style.word_spacing(), 3.0);
    assert!(style.height_override());
    assert_eq!(style.height(), 2.0);
}

#[test]
fn locale_roundtrips() {
    let mut style = TextStyle::new();
    style.set_locale("en-US");
    assert_eq!(style.locale(), "en-US");
}

#[test]
fn clone_is_independent() {
    let mut style = TextStyle::new();
    style.set_font_size(20.0);
    let cloned = style.clone_style();
    style.set_font_size(30.0);
    assert_eq!(cloned.font_size(), 20.0);
    assert_eq!(style.font_size(), 30.0);
}
