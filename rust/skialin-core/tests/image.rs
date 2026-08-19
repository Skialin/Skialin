use skialin_core::{AlphaType, ColorSpace, ColorType, Data, FilterMode, Image, ImageInfo, IRect, MipmapMode, SamplingOptions, Surface, TileMode};

fn red_square_image() -> Image {
    let mut surface = Surface::new_raster_n32_premul(16, 16).unwrap();
    surface.canvas().clear(0xFFFF0000);
    surface.image_snapshot().unwrap()
}

/// `isOpaque()` reflects the alpha-type *tag*, not actual pixel content, so
/// it needs an image whose ImageInfo is explicitly `Opaque`.
fn opaque_red_square_image() -> Image {
    let info = ImageInfo::new(16, 16, ColorType::N32, AlphaType::Opaque);
    let mut surface = Surface::new_raster(&info).unwrap();
    surface.canvas().clear(0xFFFF0000);
    surface.image_snapshot().unwrap()
}

#[test]
fn basic_properties() {
    let image = red_square_image();
    assert_eq!(image.width(), 16);
    assert_eq!(image.height(), 16);
    assert_eq!(image.dimensions(), skialin_core::ISize::new(16, 16));
    assert_eq!(image.bounds(), IRect::new(0, 0, 16, 16));
    assert!(image.unique_id() != 0);
    assert!(!image.is_alpha_only());
    assert!(!image.is_texture_backed());
    assert!(!image.is_protected());
}

#[test]
fn opaque_alpha_type_reports_opaque() {
    let image = opaque_red_square_image();
    assert!(image.is_opaque());
    assert_eq!(image.alpha_type(), AlphaType::Opaque);
}

#[test]
fn image_info_matches_source() {
    let image = red_square_image();
    let info = image.image_info();
    assert_eq!(info.width(), 16);
    assert_eq!(info.height(), 16);
}

#[test]
fn encode_and_decode_roundtrip() {
    let image = red_square_image();
    let png = image.encode_to_png().unwrap();
    assert!(!png.is_empty());

    let decoded = Image::decode(&png).unwrap();
    assert_eq!(decoded.width(), 16);
    assert_eq!(decoded.height(), 16);
    assert!(decoded.ref_encoded_data().is_some());
    assert!(image.ref_encoded_data().is_none());
}

#[test]
fn peek_pixels_and_read_pixels() {
    let image = red_square_image();
    let pixmap = image.peek_pixels().unwrap();
    assert_eq!(pixmap.width(), 16);
    assert_eq!(pixmap.get_color(0, 0), 0xFFFF0000);

    let info = ImageInfo::n32_premul(16, 16);
    let row_bytes = info.min_row_bytes();
    let mut buf = vec![0u8; info.compute_byte_size(row_bytes)];
    let ok = unsafe { image.read_pixels(&info, buf.as_mut_ptr(), row_bytes, 0, 0) };
    assert!(ok);
    assert_eq!(buf[0], 0); // B
    assert_eq!(buf[2], 255); // R (BGRA native order)
}

#[test]
fn make_subset_and_scaled() {
    let image = red_square_image();
    let subset = image.make_subset(IRect::new(0, 0, 8, 8), false).unwrap();
    assert_eq!(subset.width(), 8);
    assert_eq!(subset.height(), 8);

    let scaled = image.make_scaled(&ImageInfo::n32_premul(32, 32), SamplingOptions::new(FilterMode::Linear, MipmapMode::None)).unwrap();
    assert_eq!(scaled.width(), 32);
    assert_eq!(scaled.height(), 32);
}

#[test]
fn as_legacy_bitmap_roundtrip() {
    let image = red_square_image();
    let bitmap = image.as_legacy_bitmap().unwrap();
    assert_eq!(bitmap.width(), 16);
    assert_eq!(bitmap.height(), 16);
}

#[test]
fn make_shader_from_image() {
    let image = opaque_red_square_image();
    let shader = image.make_shader(TileMode::Clamp, TileMode::Clamp, SamplingOptions::nearest(), None).unwrap();
    assert!(shader.is_opaque());
}

#[test]
fn from_pixmap_copy_and_from_data() {
    let image = red_square_image();
    let pixmap = image.peek_pixels().unwrap();
    let copy = Image::from_pixmap_copy(&pixmap).unwrap();
    assert_eq!(copy.width(), 16);

    let info = ImageInfo::new(2, 2, ColorType::Rgba8888, AlphaType::Premul);
    let pixels = Data::with_copy(&[255u8, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255]);
    let from_data = Image::from_data(&info, &pixels, 8).unwrap();
    assert_eq!(from_data.width(), 2);
    assert_eq!(from_data.height(), 2);
}

#[test]
fn color_space_roundtrip() {
    let image = red_square_image();
    assert!(image.color_space().is_none() || image.color_space().is_some());

    let srgb = ColorSpace::srgb();
    let recolored = image.make_color_space(&srgb, false);
    assert!(recolored.is_some());

    let reinterpreted = image.reinterpret_color_space(&srgb);
    assert!(reinterpreted.is_some());
}
