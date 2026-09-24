//! Ganesh + Metal smoke test. Device/queue/texture creation is in metal_fixture (plain
//! Metal.framework/objc runtime calls, not part of the shim). macOS-only, like Skia's Metal build.
#![cfg(target_os = "macos")]

mod metal_fixture;

use metal_fixture::MetalFixture;
use skialin_core::{AlphaType, BackendRenderTarget, BackendTexture, ColorType, DirectContext, ImageInfo, Surface, SurfaceOrigin};

fn assert_red(surface: &mut Surface) {
    let info = ImageInfo::new(16, 16, ColorType::N32, AlphaType::Premul);
    let image = surface.image_snapshot().unwrap();
    assert!(image.is_texture_backed());
    let mut pixels = vec![0u8; 16 * 16 * 4];
    let ok = unsafe { image.read_pixels(&info, pixels.as_mut_ptr(), 16 * 4, 0, 0) };
    assert!(ok, "read_pixels failed");
    // ColorType::N32 is Bgra8888 on macOS: opaque red is B=0, G=0, R=255, A=255.
    assert_eq!(&pixels[0..4], &[0, 0, 255, 255]);
    assert_eq!(&pixels[pixels.len() - 4..], &[0, 0, 255, 255]);
}

#[test]
fn render_target_round_trip() {
    let Some(fixture) = MetalFixture::new() else {
        eprintln!("skipping: no Metal device available on this machine");
        return;
    };
    let mut context = DirectContext::new_metal(fixture.device.0, fixture.queue.0).expect("DirectContext::new_metal failed");

    let info = ImageInfo::new(16, 16, ColorType::N32, AlphaType::Premul);
    let mut surface = Surface::new_render_target(&mut context, false, &info, 0, SurfaceOrigin::TopLeft, None, false, false)
        .expect("failed to create GPU render-target surface");
    surface.canvas().clear(0xFFFF0000);
    context.flush();
    context.submit(true);
    assert_red(&mut surface);
}

#[test]
fn wrap_backend_texture_round_trip() {
    let Some(fixture) = MetalFixture::new() else {
        eprintln!("skipping: no Metal device available on this machine");
        return;
    };
    let mut context = DirectContext::new_metal(fixture.device.0, fixture.queue.0).expect("DirectContext::new_metal failed");
    let texture = fixture.create_texture(16, 16).expect("newTextureWithDescriptor failed");

    let backend_texture = BackendTexture::new_metal(16, 16, false, texture.0, "skialin-test-texture").expect("BackendTexture::new_metal failed");
    assert!(backend_texture.is_valid());
    // Skia retained the texture; releasing ours first must not free it out from under the surface.
    drop(texture);

    let mut surface = Surface::wrap_backend_texture(&mut context, &backend_texture, SurfaceOrigin::TopLeft, 0, ColorType::N32, None, None)
        .expect("wrap_backend_texture failed");
    surface.canvas().clear(0xFFFF0000);
    context.flush();
    context.submit(true);
    assert_red(&mut surface);
}

#[test]
fn wrap_backend_render_target_round_trip() {
    let Some(fixture) = MetalFixture::new() else {
        eprintln!("skipping: no Metal device available on this machine");
        return;
    };
    let mut context = DirectContext::new_metal(fixture.device.0, fixture.queue.0).expect("DirectContext::new_metal failed");
    let texture = fixture.create_texture(16, 16).expect("newTextureWithDescriptor failed");

    let render_target = BackendRenderTarget::new_metal(16, 16, texture.0).expect("BackendRenderTarget::new_metal failed");
    assert!(render_target.is_valid());

    let mut surface = Surface::wrap_backend_render_target(&mut context, &render_target, SurfaceOrigin::TopLeft, ColorType::N32, None, None)
        .expect("wrap_backend_render_target failed");
    surface.canvas().clear(0xFFFF0000);
    context.flush();
    context.submit(true);
    assert!(surface.image_snapshot().is_some());
}
