//! Graphite + Metal smoke test. Device/queue/texture creation is in metal_fixture (plain
//! Metal.framework/objc runtime calls, not part of the shim). macOS-only, like Skia's Metal build.
#![cfg(target_os = "macos")]

mod metal_fixture;

use metal_fixture::MetalFixture;
use skialin_core::{AlphaType, ColorType, GraphiteBackendTexture, GraphiteContext, ImageInfo, Surface};

#[test]
fn render_target_round_trip() {
    let Some(fixture) = MetalFixture::new() else {
        eprintln!("skipping: no Metal device available on this machine");
        return;
    };
    let mut context = GraphiteContext::new_metal(fixture.device.0, fixture.queue.0).expect("GraphiteContext::new_metal failed");
    let mut recorder = context.make_recorder().expect("make_recorder failed");

    let info = ImageInfo::new(16, 16, ColorType::N32, AlphaType::Premul);
    let mut surface = Surface::new_graphite_render_target(&mut recorder, &info, false, None).expect("failed to create Graphite render-target surface");
    surface.canvas().clear(0xFFFF0000);

    let mut recording = recorder.snap().expect("snap failed");
    let status = context.insert_recording(&mut recording, &mut surface);
    assert_eq!(status, 0, "insertRecording failed with status {status}");
    assert!(context.submit(true), "submit failed");

    // Pipeline only, not pixel contents: see graphite_vulkan.rs for why Graphite readback isn't
    // tested yet.
    let image = surface.image_snapshot().unwrap();
    assert!(image.is_texture_backed());
}

#[test]
fn wrap_texture_round_trip() {
    let Some(fixture) = MetalFixture::new() else {
        eprintln!("skipping: no Metal device available on this machine");
        return;
    };
    let mut context = GraphiteContext::new_metal(fixture.device.0, fixture.queue.0).expect("GraphiteContext::new_metal failed");
    let mut recorder = context.make_recorder().expect("make_recorder failed");
    // Graphite's BackendTexture doesn't retain this, so it stays alive (in `texture`) until the end.
    let texture = fixture.create_texture(16, 16).expect("newTextureWithDescriptor failed");

    let backend_texture = GraphiteBackendTexture::new_metal(16, 16, texture.0).expect("GraphiteBackendTexture::new_metal failed");
    assert!(backend_texture.is_valid());

    let mut surface = Surface::wrap_graphite_backend_texture(&mut recorder, &backend_texture, ColorType::N32, None, None)
        .expect("wrap_graphite_backend_texture failed");
    assert_eq!(surface.width(), 16);
    surface.canvas().clear(0xFFFF0000);

    let mut recording = recorder.snap().expect("snap failed");
    let status = context.insert_recording(&mut recording, &mut surface);
    assert_eq!(status, 0, "insertRecording failed with status {status}");
    assert!(context.submit(true), "submit failed");

    drop(surface);
    drop(backend_texture);
    drop(texture);
}
