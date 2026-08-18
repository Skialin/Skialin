use skialin_core::{FontCollection, FontMgr, ParagraphBuilder, ParagraphStyle, PlaceholderAlignment, PlaceholderBaseline, PlaceholderStyle, StrutStyle, TextStyle};

fn make_font_collection() -> FontCollection {
    let mut collection = FontCollection::new();
    collection.set_default_font_manager(&FontMgr::system());
    collection
}

#[test]
fn strut_style_defaults_and_roundtrips() {
    let mut strut = StrutStyle::new();
    assert!(!strut.strut_enabled());

    strut.set_strut_enabled(true);
    strut.set_font_size(20.0);
    strut.set_height(1.5);
    strut.set_height_override(true);
    strut.set_leading(0.2);
    strut.set_force_strut_height(true);
    strut.set_half_leading(true);
    strut.set_font_families(&["Arial"]);

    assert!(strut.strut_enabled());
    assert_eq!(strut.font_size(), 20.0);
    assert_eq!(strut.height(), 1.5);
    assert!(strut.height_override());
    assert_eq!(strut.leading(), 0.2);
    assert!(strut.force_strut_height());
    assert!(strut.half_leading());
    assert_eq!(strut.font_families(), vec!["Arial".to_string()]);
}

#[test]
fn paragraph_style_strut_style_roundtrips() {
    let mut style = ParagraphStyle::new();
    let mut strut = style.strut_style();
    strut.set_strut_enabled(true);
    strut.set_font_size(22.0);
    style.set_strut_style(&strut);
    assert_eq!(style.strut_style().font_size(), 22.0);
    assert!(style.strut_style().strut_enabled());
}

#[test]
fn add_placeholder_builds_without_crashing() {
    let mut collection = make_font_collection();
    let style = ParagraphStyle::new();
    let mut builder = ParagraphBuilder::new(&style, &mut collection);
    let mut text_style = TextStyle::new();
    text_style.set_font_size(18.0);
    builder.push_style(&text_style);
    builder.add_text("Before ");
    builder.add_placeholder(PlaceholderStyle {
        width: 20.0,
        height: 20.0,
        alignment: PlaceholderAlignment::Middle,
        baseline: PlaceholderBaseline::Alphabetic,
        baseline_offset: 0.0,
    });
    builder.add_text(" after");
    builder.pop();

    let mut paragraph = builder.build();
    paragraph.layout(200.0);
    assert!(paragraph.height() > 0.0);
}
