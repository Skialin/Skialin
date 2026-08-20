use skialin_core::{AlphaType, Bitmap, ColorType, ImageInfo};

#[test]
fn install_pixels_sets_info_and_contents() {
    let info = ImageInfo::new(2, 2, ColorType::N32, AlphaType::Premul);
    let pixels: [u8; 16] = [
        0xFF, 0x00, 0x00, 0xFF, //
        0x00, 0xFF, 0x00, 0xFF, //
        0x00, 0x00, 0xFF, 0xFF, //
        0xFF, 0xFF, 0xFF, 0xFF, //
    ];
    let mut bitmap = Bitmap::new();
    let row_bytes = info.min_row_bytes();
    assert!(bitmap.install_pixels(&info, &pixels, row_bytes));

    assert_eq!(bitmap.width(), 2);
    assert_eq!(bitmap.height(), 2);
    assert_eq!(bitmap.pixels(), &pixels);
}

#[test]
fn install_pixels_replaces_prior_contents() {
    let info = ImageInfo::n32_premul(1, 1);
    let mut bitmap = Bitmap::new();
    bitmap.alloc_pixels(&info);
    bitmap.erase_color(0xFFFF0000);

    let replacement: [u8; 4] = [0x11, 0x22, 0x33, 0x44];
    let row_bytes = info.min_row_bytes();
    assert!(bitmap.install_pixels(&info, &replacement, row_bytes));
    assert_eq!(bitmap.pixels(), &replacement);
}

#[test]
fn install_pixels_rejects_too_small_row_bytes() {
    let info = ImageInfo::n32_premul(4, 4);
    let mut bitmap = Bitmap::new();
    let pixels = vec![0u8; info.compute_min_byte_size()];
    assert!(!bitmap.install_pixels(&info, &pixels, 1));
}
