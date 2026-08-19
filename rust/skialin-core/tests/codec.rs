use skialin_core::{AlphaType, Codec, Color, ColorType, Data, Image, ImageInfo};

fn make_solid_image(color: Color) -> Image {
    let info = ImageInfo::n32_premul(8, 8);
    let pixels: Vec<u8> = (0..8 * 8).flat_map(|_| color.to_le_bytes()).collect();
    let data = Data::with_copy(&pixels);
    Image::from_data(&info, &data, 8 * 4).unwrap()
}

#[test]
fn encodes_and_decodes_jpeg() {
    let image = make_solid_image(0xFF0000FF);
    let bytes = image.encode_to_jpeg(90).expect("jpeg encode should succeed");
    assert!(!bytes.is_empty());

    let codec = Codec::from_bytes(&bytes).expect("should decode as a codec");
    assert_eq!(codec.dimensions(), (8, 8));
    assert_eq!(codec.encoded_format(), 3); // SkEncodedImageFormat::kJPEG
    assert_eq!(codec.frame_count(), 1);
}

#[test]
fn encodes_and_decodes_webp() {
    let image = make_solid_image(0xFF00FF00);
    let bytes = image.encode_to_webp(90.0, false).expect("webp encode should succeed");
    assert!(!bytes.is_empty());

    let mut codec = Codec::from_bytes(&bytes).expect("should decode as a codec");
    assert_eq!(codec.dimensions(), (8, 8));
    assert_eq!(codec.encoded_format(), 6); // SkEncodedImageFormat::kWEBP

    let info = ImageInfo::new(8, 8, ColorType::N32, AlphaType::Premul);
    let mut pixels = vec![0u8; 8 * 8 * 4];
    let ok = unsafe { codec.get_pixels(&info, pixels.as_mut_ptr(), 8 * 4, 0) };
    assert!(ok);
}

#[test]
fn invalid_bytes_returns_none() {
    assert!(Codec::from_bytes(b"not an image").is_none());
}
