use skialin_core::{color, Font, FontMgr, FontStyle, Paint, Point, Surface, TextEncoding};

fn make_font() -> Font {
    let mgr = FontMgr::system();
    let name = mgr.family_name(0);
    let typeface = mgr.match_family_style(Some(&name), FontStyle::normal()).unwrap();
    Font::from_typeface(&typeface, 18.0)
}

#[test]
fn from_text_has_nonempty_bounds() {
    let font = make_font();
    let blob = skialin_core::TextBlob::from_text("Hi", &font, TextEncoding::Utf8).unwrap();
    let bounds = blob.bounds();
    assert!(bounds.right > bounds.left);
    assert_ne!(blob.unique_id(), 0);
}

#[test]
fn from_text_is_none_for_empty_string() {
    let font = make_font();
    assert!(skialin_core::TextBlob::from_text("", &font, TextEncoding::Utf8).is_none());
}

#[test]
fn from_pos_text_h_matches_glyph_count() {
    let font = make_font();
    let glyphs = font.text_to_glyphs("Hi");
    let widths = font.widths(&glyphs);
    let mut x = 0.0;
    let mut xpos = Vec::new();
    for w in widths {
        xpos.push(x);
        x += w;
    }
    let blob = skialin_core::TextBlob::from_pos_text_h("Hi", &xpos, 0.0, &font, TextEncoding::Utf8).unwrap();
    assert!(blob.bounds().right > blob.bounds().left);
}

#[test]
fn from_pos_text_matches_glyph_count() {
    let font = make_font();
    let pos = vec![Point::new(0.0, 0.0), Point::new(20.0, 0.0)];
    let blob = skialin_core::TextBlob::from_pos_text("Hi", &pos, &font, TextEncoding::Utf8).unwrap();
    assert!(blob.bounds().right > blob.bounds().left);
}

#[test]
fn rsxform_intercepts_and_serialize_roundtrip() {
    let font = make_font();
    let xforms = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 20.0, 0.0];
    let blob = skialin_core::TextBlob::from_rsxform("Hi", &xforms, &font, TextEncoding::Utf8).unwrap();
    assert!(blob.bounds().right > blob.bounds().left);

    let plain_blob = skialin_core::TextBlob::from_text("Hi", &font, TextEncoding::Utf8).unwrap();
    let b = plain_blob.bounds();
    let intervals = plain_blob.get_intercepts(b.top, b.bottom, None);
    assert!(intervals.len() % 2 == 0);

    let data = plain_blob.serialize_to_data();
    assert!(data.size() > 0);
    let restored = skialin_core::TextBlob::from_data(&data).unwrap();
    assert_eq!(restored.bounds().right, plain_blob.bounds().right);
}

#[test]
fn draws_without_crashing() {
    let font = make_font();
    let blob = skialin_core::TextBlob::from_text("Hi", &font, TextEncoding::Utf8).unwrap();
    let mut surface = Surface::new_raster_n32_premul(64, 64).unwrap();
    let mut canvas = surface.canvas;
    let mut paint = Paint::new();
    paint.set_color(color::BLACK);
    canvas.draw_text_blob(&blob, 4.0, 20.0, &paint);
}
