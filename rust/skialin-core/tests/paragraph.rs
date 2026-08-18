use skialin_core::{color, FontCollection, FontMgr, ParagraphBuilder, ParagraphStyle, Surface, TextDirection, TextStyle};

fn make_font_collection() -> FontCollection {
    let mut collection = FontCollection::new();
    collection.set_default_font_manager(&FontMgr::system());
    collection
}

#[test]
fn layout_produces_nonzero_height() {
    let mut collection = make_font_collection();
    let style = ParagraphStyle::new();
    let mut builder = ParagraphBuilder::new(&style, &mut collection);
    let mut text_style = TextStyle::new();
    text_style.set_font_size(18.0);
    text_style.set_color(color::BLACK);
    builder.push_style(&text_style);
    builder.add_text("Hello, world!");
    builder.pop();

    let mut paragraph = builder.build();
    paragraph.layout(200.0);

    assert!(paragraph.height() > 0.0);
    assert!(paragraph.max_width() <= 200.0);
    assert_eq!(paragraph.line_number(), 1);
    assert!(!paragraph.did_exceed_max_lines());
}

#[test]
fn narrow_width_wraps_to_multiple_lines() {
    let mut collection = make_font_collection();
    let style = ParagraphStyle::new();
    let mut builder = ParagraphBuilder::new(&style, &mut collection);
    let mut text_style = TextStyle::new();
    text_style.set_font_size(18.0);
    builder.push_style(&text_style);
    builder.add_text("Hello, world! This is a longer sentence that should wrap.");
    builder.pop();

    let mut paragraph = builder.build();
    paragraph.layout(80.0);

    assert!(paragraph.line_number() > 1);
}

#[test]
fn line_metrics_cover_the_text() {
    let mut collection = make_font_collection();
    let style = ParagraphStyle::new();
    let mut builder = ParagraphBuilder::new(&style, &mut collection);
    let mut text_style = TextStyle::new();
    text_style.set_font_size(18.0);
    builder.push_style(&text_style);
    let text = "Hello";
    builder.add_text(text);
    builder.pop();

    let mut paragraph = builder.build();
    paragraph.layout(200.0);

    let metrics = paragraph.line_metrics();
    assert_eq!(metrics.len(), 1);
    assert_eq!(metrics[0].start_index, 0);
    assert_eq!(metrics[0].end_index, text.len());
    assert!(metrics[0].width > 0.0);
    assert!(metrics[0].height > 0.0);
}

#[test]
fn glyph_position_at_coordinate_is_within_text_range() {
    let mut collection = make_font_collection();
    let style = ParagraphStyle::new();
    let mut builder = ParagraphBuilder::new(&style, &mut collection);
    let mut text_style = TextStyle::new();
    text_style.set_font_size(18.0);
    builder.push_style(&text_style);
    let text = "Hello";
    builder.add_text(text);
    builder.pop();

    let mut paragraph = builder.build();
    paragraph.layout(200.0);

    let pos = paragraph.glyph_position_at_coordinate(0.0, 5.0);
    assert!(pos.position >= 0);
    assert!((pos.position as usize) <= text.len());
}

#[test]
fn rtl_direction_lays_out_without_crashing() {
    let mut collection = make_font_collection();
    let mut style = ParagraphStyle::new();
    style.set_text_direction(TextDirection::Rtl);
    let mut builder = ParagraphBuilder::new(&style, &mut collection);
    let mut text_style = TextStyle::new();
    text_style.set_font_size(18.0);
    builder.push_style(&text_style);
    builder.add_text("مرحبا بالعالم");
    builder.pop();

    let mut paragraph = builder.build();
    paragraph.layout(200.0);
    assert!(paragraph.height() > 0.0);
}

#[test]
fn paints_without_crashing() {
    let mut collection = make_font_collection();
    let style = ParagraphStyle::new();
    let mut builder = ParagraphBuilder::new(&style, &mut collection);
    let mut text_style = TextStyle::new();
    text_style.set_font_size(18.0);
    text_style.set_color(color::BLACK);
    builder.push_style(&text_style);
    builder.add_text("Hello, world!");
    builder.pop();

    let mut paragraph = builder.build();
    paragraph.layout(200.0);

    let mut surface = Surface::new_raster_n32_premul(200, 100).unwrap();
    let mut canvas = surface.canvas();
    paragraph.paint(&mut canvas, 0.0, 0.0);
}
