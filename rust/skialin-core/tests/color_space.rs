use skialin_core::{named_gamut, named_transfer_fn, CicpPrimaries, CicpTransferFn, ColorSpace};

#[test]
fn srgb_is_srgb() {
    let cs = ColorSpace::srgb();
    assert!(cs.is_srgb());
    assert!(cs.gamma_close_to_srgb());
    assert!(!cs.gamma_is_linear());
}

#[test]
fn srgb_linear_has_linear_gamma() {
    let cs = ColorSpace::srgb_linear();
    assert!(cs.gamma_is_linear());
    assert!(!cs.is_srgb());
}

#[test]
fn rgb_roundtrips_gamut() {
    let cs = ColorSpace::rgb(named_transfer_fn::SRGB, named_gamut::SRGB);
    let xyz = cs.to_xyz_d50().unwrap();
    for (a, b) in xyz.iter().zip(named_gamut::SRGB.iter()) {
        assert!((a - b).abs() < 1e-4, "{a} vs {b}");
    }
}

#[test]
fn cicp_srgb_matches_make_srgb() {
    let a = ColorSpace::cicp(CicpPrimaries::Rec709, CicpTransferFn::Srgb).unwrap();
    let b = ColorSpace::srgb();
    assert!(a.equals(&b));
}

#[test]
fn make_linear_gamma_produces_linear() {
    let cs = ColorSpace::srgb().make_linear_gamma();
    assert!(cs.gamma_is_linear());
}

#[test]
fn make_srgb_gamma_produces_srgb_gamma() {
    let cs = ColorSpace::srgb_linear().make_srgb_gamma();
    assert!(cs.gamma_close_to_srgb());
}

#[test]
fn color_spin_is_not_srgb() {
    let cs = ColorSpace::srgb().make_color_spin();
    assert!(!cs.is_srgb());
}

#[test]
fn serialize_then_deserialize_roundtrips() {
    let cs = ColorSpace::srgb();
    let data = cs.serialize();
    let restored = ColorSpace::deserialize(data.as_bytes()).unwrap();
    assert!(cs.equals(&restored));
}

#[test]
fn pq_transfer_fn_is_not_numerical() {
    let cs = ColorSpace::rgb(named_transfer_fn::PQ, named_gamut::REC2020);
    assert!(cs.numerical_transfer_fn().is_none());
}

#[test]
fn linear_transfer_fn_is_numerical() {
    let cs = ColorSpace::rgb(named_transfer_fn::LINEAR, named_gamut::SRGB);
    assert!(cs.numerical_transfer_fn().is_some());
}

#[test]
fn from_icc_profile_rejects_garbage() {
    assert!(ColorSpace::from_icc_profile(b"not an icc profile").is_none());
}

#[test]
fn gamut_transform_identity_is_identity_matrix() {
    let cs = ColorSpace::srgb();
    let m = cs.gamut_transform_to(&cs);
    assert!((m[0] - 1.0).abs() < 1e-4);
    assert!((m[4] - 1.0).abs() < 1e-4);
    assert!((m[8] - 1.0).abs() < 1e-4);
}
