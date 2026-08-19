use skialin_core::{FontStyle, Slant, Typeface};

#[test]
fn empty_typeface_has_no_glyphs() {
    let typeface = Typeface::empty();
    assert_eq!(typeface.count_glyphs(), 0);
    assert!(!typeface.is_bold());
    assert!(!typeface.is_italic());
}

#[test]
fn empty_typeface_has_a_unique_id() {
    let a = Typeface::empty();
    let b = Typeface::empty();
    assert_ne!(a.unique_id(), 0);
    assert_ne!(b.unique_id(), 0);
}

#[test]
fn font_style_roundtrips_normal() {
    let typeface = Typeface::empty();
    let style = typeface.font_style();
    assert_eq!(style, FontStyle::normal());
    assert_eq!(style.slant, Slant::Upright);
}

#[test]
fn family_name_is_empty_string_for_empty_typeface() {
    let typeface = Typeface::empty();
    assert_eq!(typeface.family_name(), "");
}

#[test]
fn table_tags_are_consistent_with_table_size() {
    let mgr = skialin_core::FontMgr::system();
    let name = mgr.family_name(0);
    let typeface = mgr.match_family_style(Some(&name), FontStyle::normal()).unwrap();
    for tag in typeface.table_tags() {
        let size = typeface.table_size(tag);
        let data = typeface.table_data(tag, 0, size);
        assert_eq!(data.len(), size);
    }
}
