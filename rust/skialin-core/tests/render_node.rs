use skialin_core::canvas::ClipOp;
use skialin_core::{color, AlphaType, ColorType, ImageInfo, Paint, PathBuilder, PathDirection, Rect, RenderNode, RenderNodeContext, Surface};

#[test]
fn record_and_draw_matches_pixels() {
    let context = RenderNodeContext::new(false, true);
    let mut node = RenderNode::new(context);
    node.set_bounds(Rect::new(0.0, 0.0, 16.0, 16.0));
    {
        let mut canvas = node.begin_recording();
        let mut paint = Paint::new();
        paint.set_color(color::RED);
        canvas.draw_rect(Rect::new(0.0, 0.0, 16.0, 16.0), &paint);
    }
    node.end_recording();

    let mut surface = Surface::new_raster_n32_premul(16, 16).unwrap();
    node.draw_into(&mut surface.canvas);
    let image = surface.image_snapshot().unwrap();

    let info = ImageInfo::new(16, 16, ColorType::N32, AlphaType::Premul);
    let mut pixels = vec![0u8; 16 * 16 * 4];
    assert!(unsafe { image.read_pixels(&info, pixels.as_mut_ptr(), 16 * 4, 0, 0) });
    assert_eq!(&pixels[0..4], &[0, 0, 255, 255]);
}

#[test]
fn clip_confines_content() {
    let context = RenderNodeContext::new(false, true);
    let mut node = RenderNode::new(context);
    node.set_bounds(Rect::new(0.0, 0.0, 16.0, 16.0));
    node.set_clip(true);
    node.set_clip_rect(Some(Rect::new(0.0, 0.0, 8.0, 16.0)), ClipOp::Intersect, false);
    {
        let mut canvas = node.begin_recording();
        let mut paint = Paint::new();
        paint.set_color(color::RED);
        canvas.draw_rect(Rect::new(0.0, 0.0, 16.0, 16.0), &paint);
    }
    node.end_recording();

    let mut surface = Surface::new_raster_n32_premul(16, 16).unwrap();
    surface.canvas.clear(color::WHITE);
    node.draw_into(&mut surface.canvas);
    let image = surface.image_snapshot().unwrap();

    let info = ImageInfo::new(16, 16, ColorType::N32, AlphaType::Premul);
    let mut pixels = vec![0u8; 16 * 16 * 4];
    assert!(unsafe { image.read_pixels(&info, pixels.as_mut_ptr(), 16 * 4, 0, 0) });

    let px = |x: usize, y: usize| -> [u8; 4] {
        let i = (y * 16 + x) * 4;
        [pixels[i], pixels[i + 1], pixels[i + 2], pixels[i + 3]]
    };
    assert_eq!(px(2, 8), [0, 0, 255, 255]);
    assert_eq!(px(12, 8), [255, 255, 255, 255]);
}

#[test]
fn clip_path_confines_content() {
    let context = RenderNodeContext::new(false, true);
    let mut node = RenderNode::new(context);
    node.set_bounds(Rect::new(0.0, 0.0, 16.0, 16.0));
    node.set_clip(true);
    let mut builder = PathBuilder::new();
    builder.add_rect(Rect::new(0.0, 0.0, 8.0, 16.0), PathDirection::Clockwise);
    node.set_clip_path(Some(builder.snapshot()), ClipOp::Intersect, false);
    {
        let mut canvas = node.begin_recording();
        let mut paint = Paint::new();
        paint.set_color(color::RED);
        canvas.draw_rect(Rect::new(0.0, 0.0, 16.0, 16.0), &paint);
    }
    node.end_recording();

    let mut surface = Surface::new_raster_n32_premul(16, 16).unwrap();
    surface.canvas.clear(color::WHITE);
    node.draw_into(&mut surface.canvas);
    let image = surface.image_snapshot().unwrap();

    let info = ImageInfo::new(16, 16, ColorType::N32, AlphaType::Premul);
    let mut pixels = vec![0u8; 16 * 16 * 4];
    assert!(unsafe { image.read_pixels(&info, pixels.as_mut_ptr(), 16 * 4, 0, 0) });

    let px = |x: usize, y: usize| -> [u8; 4] {
        let i = (y * 16 + x) * 4;
        [pixels[i], pixels[i + 1], pixels[i + 2], pixels[i + 3]]
    };
    assert_eq!(px(2, 8), [0, 0, 255, 255]);
    assert_eq!(px(12, 8), [255, 255, 255, 255]);
}

#[test]
fn alpha_reduces_opacity() {
    let context = RenderNodeContext::new(false, true);
    let mut node = RenderNode::new(context);
    node.set_bounds(Rect::new(0.0, 0.0, 4.0, 4.0));
    node.set_alpha(0.5);
    {
        let mut canvas = node.begin_recording();
        let mut paint = Paint::new();
        paint.set_color(color::RED);
        canvas.draw_rect(Rect::new(0.0, 0.0, 4.0, 4.0), &paint);
    }
    node.end_recording();

    let mut surface = Surface::new_raster_n32_premul(4, 4).unwrap();
    surface.canvas.clear(color::WHITE);
    node.draw_into(&mut surface.canvas);
    let image = surface.image_snapshot().unwrap();

    let info = ImageInfo::new(4, 4, ColorType::N32, AlphaType::Premul);
    let mut pixels = vec![0u8; 4 * 4 * 4];
    assert!(unsafe { image.read_pixels(&info, pixels.as_mut_ptr(), 4 * 4, 0, 0) });
    // R stays 255 either way (both src and dst are saturated red), but B goes
    // from 255 (white) toward 0 (red) -- check that to detect blending.
    let b = pixels[0];
    assert!(b > 0 && b < 255, "expected blended blue channel, got {b}");
}
