use skialin_core::{Edging, Font, FontMgr, FontStyle, Hinting};

#[test]
fn default_font_has_a_typeface() {
    let font = Font::new();
    assert!(font.typeface().is_some());
}

#[test]
fn size_roundtrips() {
    let mut font = Font::new();
    font.set_size(24.0);
    assert_eq!(font.size(), 24.0);
}

#[test]
fn edging_and_hinting_roundtrip() {
    let mut font = Font::new();
    font.set_edging(Edging::AntiAlias);
    assert_eq!(font.edging(), Edging::AntiAlias);
    font.set_hinting(Hinting::Full);
    assert_eq!(font.hinting(), Hinting::Full);
}

#[test]
fn from_typeface_uses_the_given_typeface() {
    let mgr = FontMgr::system();
    let name = mgr.family_name(0);
    let typeface = mgr.match_family_style(Some(&name), FontStyle::normal()).unwrap();
    let font = Font::from_typeface(&typeface, 18.0);
    assert_eq!(font.size(), 18.0);
    assert!(font.typeface().is_some());
}

#[test]
fn text_to_glyphs_and_measure_text_are_consistent() {
    let mgr = FontMgr::system();
    let name = mgr.family_name(0);
    let typeface = mgr.match_family_style(Some(&name), FontStyle::normal()).unwrap();
    let font = Font::from_typeface(&typeface, 18.0);

    let glyphs = font.text_to_glyphs("Hi");
    assert_eq!(glyphs.len(), 2);

    let width = font.measure_text("Hi");
    assert!(width > 0.0);

    let widths = font.widths(&glyphs);
    assert_eq!(widths.len(), 2);
    assert!(widths.iter().sum::<f32>() > 0.0);
}

#[test]
fn metrics_are_nonzero_for_a_positive_size() {
    let mgr = FontMgr::system();
    let name = mgr.family_name(0);
    let typeface = mgr.match_family_style(Some(&name), FontStyle::normal()).unwrap();
    let font = Font::from_typeface(&typeface, 18.0);

    let metrics = font.metrics();
    assert!(metrics.descent - metrics.ascent > 0.0);
    assert!(font.spacing() > 0.0);
}
