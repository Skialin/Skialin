use skialin_core::{color, Paint, PictureRecorder, Rect, Surface};

#[test]
fn record_and_playback_draws_matching_pixels() {
    let bounds = Rect::new(0.0, 0.0, 16.0, 16.0);
    let mut recorder = PictureRecorder::new();
    {
        let mut canvas = recorder.begin_recording(bounds);
        let mut paint = Paint::new();
        paint.set_color(color::RED);
        canvas.draw_rect(bounds, &paint);
    }
    let picture = recorder.finish_recording_as_picture().expect("finishRecordingAsPicture failed");
    assert_eq!(picture.cull_rect(), bounds);
    assert!(picture.approximate_op_count(false) > 0);

    let mut surface = Surface::new_raster_n32_premul(16, 16).unwrap();
    surface.canvas().draw_picture(&picture);
    let image = surface.image_snapshot().unwrap();

    let mut pixels = vec![0u8; 16 * 16 * 4];
    let info = skialin_core::ImageInfo::new(16, 16, skialin_core::ColorType::N32, skialin_core::AlphaType::Premul);
    assert!(unsafe { image.read_pixels(&info, pixels.as_mut_ptr(), 16 * 4, 0, 0) });
    // BGRA8888: opaque red -> B=0, G=0, R=255, A=255.
    assert_eq!(&pixels[0..4], &[0, 0, 255, 255]);
}
