use skialin_core::{PixelGeometry, SurfaceProps, SurfacePropsFlags};

#[test]
fn accessors_match_constructor() {
    let flags = SurfacePropsFlags::USE_DEVICE_INDEPENDENT_FONTS | SurfacePropsFlags::ALWAYS_DITHER;
    let props = SurfaceProps::new(flags, PixelGeometry::RgbH, 0.5, 1.5);
    assert_eq!(props.flags().0, flags.0);
    assert_eq!(props.pixel_geometry(), PixelGeometry::RgbH);
    assert_eq!(props.text_contrast(), 0.5);
    assert_eq!(props.text_gamma(), 1.5);
}

#[test]
fn default_flags_is_zero() {
    let props = SurfaceProps::new(SurfacePropsFlags::DEFAULT, PixelGeometry::Unknown, 0.0, 0.0);
    assert_eq!(props.flags().0, 0);
}

#[test]
fn clone_with_pixel_geometry_changes_only_geometry() {
    let props = SurfaceProps::new(SurfacePropsFlags::ALWAYS_DITHER, PixelGeometry::RgbV, 0.2, 1.8);
    let cloned = props.clone_with_pixel_geometry(PixelGeometry::BgrH);
    assert_eq!(cloned.pixel_geometry(), PixelGeometry::BgrH);
    assert_eq!(cloned.flags().0, props.flags().0);
    assert_eq!(cloned.text_contrast(), props.text_contrast());
    assert_eq!(cloned.text_gamma(), props.text_gamma());
}

#[test]
fn equals_compares_by_value() {
    let a = SurfaceProps::new(SurfacePropsFlags::DEFAULT, PixelGeometry::RgbH, 0.0, 2.2);
    let b = SurfaceProps::new(SurfacePropsFlags::DEFAULT, PixelGeometry::RgbH, 0.0, 2.2);
    let c = SurfaceProps::new(SurfacePropsFlags::DEFAULT, PixelGeometry::BgrH, 0.0, 2.2);
    assert!(a == b);
    assert!(a != c);
}

#[test]
fn clone_is_independent() {
    let props = SurfaceProps::new(SurfacePropsFlags::DEFAULT, PixelGeometry::RgbH, 0.1, 2.0);
    let cloned = props.clone();
    assert!(props == cloned);
}
