use skialin_core::{AlphaType, ColorType, ImageInfo, Rect, SVGCanvasFlags, Surface, SVGCanvas, SVGDOM};

const SIMPLE_SVG: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16">
<rect x="0" y="0" width="16" height="16" fill="#ff0000"/>
</svg>"##;

#[test]
fn parses_and_renders_without_crashing() {
    let dom = SVGDOM::from_bytes(SIMPLE_SVG).expect("should parse");
    let (width, height) = dom.container_size();
    assert_eq!((width, height), (16.0, 16.0));

    let mut surface = Surface::new_raster_n32_premul(16, 16).unwrap();
    dom.render(&mut surface.canvas);
    let image = surface.image_snapshot().unwrap();

    let info = ImageInfo::new(16, 16, ColorType::N32, AlphaType::Premul);
    let mut pixels = vec![0u8; 16 * 16 * 4];
    assert!(unsafe { image.read_pixels(&info, pixels.as_mut_ptr(), 16 * 4, 0, 0) });
    assert_eq!(&pixels[0..4], &[0, 0, 255, 255]);
}

#[test]
fn invalid_bytes_returns_none() {
    assert!(SVGDOM::from_bytes(b"not svg at all").is_none());
}

#[test]
fn svg_canvas_records_draws_as_xml() {
    let mut svg_canvas = SVGCanvas::new(Rect::new(0.0, 0.0, 16.0, 16.0), SVGCanvasFlags::default());
    {
        let mut canvas = svg_canvas.canvas;
        let mut paint = skialin_core::Paint::new();
        paint.set_color(skialin_core::color::RED);
        canvas.draw_rect(Rect::new(0.0, 0.0, 16.0, 16.0), &paint);
    }
    let data = svg_canvas.finish();
    let xml = std::str::from_utf8(data.as_bytes()).expect("valid utf8");
    assert!(xml.contains("<svg"));
    assert!(xml.contains("rect") || xml.contains("path"));
}
