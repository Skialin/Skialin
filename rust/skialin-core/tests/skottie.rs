use skialin_core::{Rect, SkottieAnimation, Surface};

const MINIMAL_LOTTIE: &[u8] = br##"{
  "v": "5.5.2", "fr": 30, "ip": 0, "op": 30, "w": 16, "h": 16, "nm": "test",
  "layers": [
    { "ty": 4, "nm": "rect", "ip": 0, "op": 30, "st": 0,
      "ks": { "o": { "a": 0, "k": 100 }, "p": { "a": 0, "k": [8, 8] }, "s": { "a": 0, "k": [100, 100] }, "r": { "a": 0, "k": 0 }, "a": { "a": 0, "k": [0, 0] } },
      "shapes": [
        { "ty": "rc", "p": { "a": 0, "k": [0, 0] }, "s": { "a": 0, "k": [16, 16] }, "r": { "a": 0, "k": 0 } },
        { "ty": "fl", "c": { "a": 0, "k": [1, 0, 0, 1] }, "o": { "a": 0, "k": 100 } }
      ]
    }
  ]
}"##;

#[test]
fn parses_and_reports_metadata() {
    let animation = SkottieAnimation::from_bytes(MINIMAL_LOTTIE).expect("should parse");
    assert_eq!(animation.size(), (16.0, 16.0));
    assert_eq!(animation.duration(), 1.0);
    assert_eq!(animation.fps(), 30.0);
}

#[test]
fn renders_without_crashing() {
    let mut animation = SkottieAnimation::from_bytes(MINIMAL_LOTTIE).expect("should parse");
    animation.seek(0.5);
    animation.seek_frame(15.0);

    let mut surface = Surface::new_raster_n32_premul(16, 16).unwrap();
    animation.render(&mut surface.canvas(), Some(Rect::new(0.0, 0.0, 16.0, 16.0)));
}

#[test]
fn invalid_bytes_returns_none() {
    assert!(SkottieAnimation::from_bytes(b"not json").is_none());
}
