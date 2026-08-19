use skialin_core::{AlphaType, ColorType, ImageInfo, IRect, Pixmap, SamplingOptions};

#[test]
fn wraps_a_buffer_and_reads_back() {
    let info = ImageInfo::new(4, 4, ColorType::Rgba8888, AlphaType::Premul);
    let row_bytes = info.min_row_bytes();
    let mut buffer = vec![0u8; info.compute_byte_size(row_bytes)];
    // pixel (1,1) = opaque red, RGBA byte order.
    let offset = row_bytes * 1 + 4 * 1;
    buffer[offset] = 255;
    buffer[offset + 3] = 255;

    let pixmap = unsafe { Pixmap::new(&info, buffer.as_ptr(), row_bytes) };
    assert_eq!(pixmap.width(), 4);
    assert_eq!(pixmap.height(), 4);
    assert!(!pixmap.is_empty());
    assert_eq!(pixmap.color_type(), ColorType::Rgba8888);
    assert_eq!(pixmap.row_bytes(), row_bytes);

    let color = pixmap.get_color(1, 1);
    assert_eq!(color, 0xFFFF0000);

    let subset = unsafe { pixmap.extract_subset(IRect::new(1, 1, 3, 3)) }.unwrap();
    assert_eq!(subset.width(), 2);
    assert_eq!(subset.height(), 2);
    assert_eq!(subset.get_color(0, 0), 0xFFFF0000);

    assert!(unsafe { pixmap.extract_subset(IRect::new(10, 10, 20, 20)) }.is_none());
}

#[test]
fn read_pixels_copies_into_destination() {
    let info = ImageInfo::new(4, 4, ColorType::Rgba8888, AlphaType::Premul);
    let row_bytes = info.min_row_bytes();
    let mut buffer = vec![0u8; info.compute_byte_size(row_bytes)];
    let offset = row_bytes * 1 + 4 * 1;
    buffer[offset] = 255;
    buffer[offset + 3] = 255;
    let src = unsafe { Pixmap::new(&info, buffer.as_ptr(), row_bytes) };

    let dst_info = ImageInfo::new(2, 2, ColorType::Rgba8888, AlphaType::Premul);
    let dst_row_bytes = dst_info.min_row_bytes();
    let mut dst_buffer = vec![0u8; dst_info.compute_byte_size(dst_row_bytes)];
    let mut dst = unsafe { Pixmap::new(&dst_info, dst_buffer.as_mut_ptr(), dst_row_bytes) };

    assert!(src.read_pixels(&mut dst, 1, 1));
    assert_eq!(dst.get_color(0, 0), 0xFFFF0000);
}

#[test]
fn scale_pixels_fills_destination() {
    let info = ImageInfo::new(4, 4, ColorType::Rgba8888, AlphaType::Premul);
    let row_bytes = info.min_row_bytes();
    let mut buffer = vec![0u8; info.compute_byte_size(row_bytes)];
    for chunk in buffer.chunks_exact_mut(4) {
        chunk[0] = 255;
        chunk[3] = 255;
    }
    let src = unsafe { Pixmap::new(&info, buffer.as_ptr(), row_bytes) };

    let dst_info = ImageInfo::new(2, 2, ColorType::Rgba8888, AlphaType::Premul);
    let dst_row_bytes = dst_info.min_row_bytes();
    let mut dst_buffer = vec![0u8; dst_info.compute_byte_size(dst_row_bytes)];
    let mut dst = unsafe { Pixmap::new(&dst_info, dst_buffer.as_mut_ptr(), dst_row_bytes) };

    assert!(src.scale_pixels(&mut dst, SamplingOptions::nearest()));
    assert_eq!(dst.get_color(0, 0), 0xFFFF0000);
}
