use skialin_core::{ParagraphStyle, TextAlign, TextDirection, TextHeightBehavior};

#[test]
fn defaults_to_ltr() {
    let style = ParagraphStyle::new();
    assert_eq!(style.text_direction(), TextDirection::Ltr);
}

#[test]
fn text_direction_roundtrips_to_rtl() {
    let mut style = ParagraphStyle::new();
    style.set_text_direction(TextDirection::Rtl);
    assert_eq!(style.text_direction(), TextDirection::Rtl);
}

#[test]
fn text_align_roundtrips() {
    let mut style = ParagraphStyle::new();
    style.set_text_align(TextAlign::Justify);
    assert_eq!(style.text_align(), TextAlign::Justify);
}

#[test]
fn max_lines_and_ellipsis_roundtrip() {
    let mut style = ParagraphStyle::new();
    style.set_max_lines(3);
    style.set_ellipsis("...");
    assert_eq!(style.max_lines(), 3);
    assert_eq!(style.ellipsis(), "...");
}

#[test]
fn height_and_behavior_roundtrip() {
    let mut style = ParagraphStyle::new();
    style.set_height(1.5);
    style.set_text_height_behavior(TextHeightBehavior::DISABLE_FIRST_ASCENT);
    assert_eq!(style.height(), 1.5);
    assert_eq!(style.text_height_behavior(), TextHeightBehavior::DISABLE_FIRST_ASCENT);
}

#[test]
fn text_style_roundtrips() {
    let mut style = ParagraphStyle::new();
    let mut text_style = style.text_style();
    text_style.set_font_size(22.0);
    style.set_text_style(&text_style);
    assert_eq!(style.text_style().font_size(), 22.0);
}
