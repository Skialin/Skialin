use skialin_core::{FontMgr, FontStyle};

#[test]
fn system_font_mgr_has_families() {
    let mgr = FontMgr::system();
    assert!(mgr.count_families() > 0);
}

#[test]
fn family_name_is_non_empty_for_a_valid_index() {
    let mgr = FontMgr::system();
    let name = mgr.family_name(0);
    assert!(!name.is_empty());
}

#[test]
fn match_family_style_finds_a_typeface_for_a_known_family() {
    let mgr = FontMgr::system();
    let name = mgr.family_name(0);
    let typeface = mgr.match_family_style(Some(&name), FontStyle::normal());
    assert!(typeface.is_some());
}

#[test]
fn match_family_style_returns_none_for_empty_mgr() {
    let mgr = FontMgr::empty();
    let typeface = mgr.match_family_style(Some("Nonexistent Family XYZ"), FontStyle::normal());
    assert!(typeface.is_none());
}

#[test]
fn make_from_file_returns_none_for_missing_file() {
    let mgr = FontMgr::system();
    let typeface = mgr.make_from_file("C:/definitely/not/a/real/font.ttf", 0);
    assert!(typeface.is_none());
}
