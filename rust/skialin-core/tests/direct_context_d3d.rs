//! Ganesh + Direct3D 12 smoke test. Adapter/device/queue creation uses the `windows` crate (plain
//! D3D12/DXGI bindings, not part of the shim). Windows-only, like Skia's D3D backend itself.
#![cfg(windows)]

use skialin_core::{AlphaType, BackendTexture, ColorType, DirectContext, ImageInfo, Surface, SurfaceOrigin};
use windows::core::Interface;
use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_11_0;
use windows::Win32::Graphics::Direct3D12::{
    D3D12CreateDevice, ID3D12CommandQueue, ID3D12Device, ID3D12Resource, D3D12_COMMAND_LIST_TYPE_DIRECT, D3D12_COMMAND_QUEUE_DESC,
    D3D12_HEAP_FLAG_NONE, D3D12_HEAP_PROPERTIES, D3D12_HEAP_TYPE_DEFAULT, D3D12_RESOURCE_DESC, D3D12_RESOURCE_DIMENSION_TEXTURE2D,
    D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET, D3D12_RESOURCE_STATE_COMMON, D3D12_TEXTURE_LAYOUT_UNKNOWN,
};
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_SAMPLE_DESC};
use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory1, IDXGIAdapter1, IDXGIFactory4, DXGI_ADAPTER_FLAG_SOFTWARE};

struct D3DFixture {
    adapter: IDXGIAdapter1,
    device: ID3D12Device,
    queue: ID3D12CommandQueue,
}

impl D3DFixture {
    /// The first hardware adapter that can make a feature-level-11 device, or None (skip) when the
    /// machine has none -- e.g. a CI runner without a GPU.
    fn new() -> Option<Self> {
        let factory: IDXGIFactory4 = unsafe { CreateDXGIFactory1() }.ok()?;
        for index in 0.. {
            let adapter = unsafe { factory.EnumAdapters1(index) }.ok()?;
            let desc = unsafe { adapter.GetDesc1() }.ok()?;
            if desc.Flags & DXGI_ADAPTER_FLAG_SOFTWARE.0 as u32 != 0 {
                continue;
            }
            let mut device: Option<ID3D12Device> = None;
            if unsafe { D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_11_0, &mut device) }.is_err() {
                continue;
            }
            let device = device?;
            let queue_desc = D3D12_COMMAND_QUEUE_DESC { Type: D3D12_COMMAND_LIST_TYPE_DIRECT, ..Default::default() };
            let queue: ID3D12CommandQueue = unsafe { device.CreateCommandQueue(&queue_desc) }.ok()?;
            return Some(D3DFixture { adapter, device, queue });
        }
        None
    }

    fn direct_context(&self) -> Option<DirectContext> {
        DirectContext::new_d3d(self.adapter.as_raw(), self.device.as_raw(), self.queue.as_raw(), false)
    }

    fn create_texture(&self, width: u32, height: u32) -> Option<ID3D12Resource> {
        let heap = D3D12_HEAP_PROPERTIES { Type: D3D12_HEAP_TYPE_DEFAULT, ..Default::default() };
        let desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
            Width: width as u64,
            Height: height,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: DXGI_FORMAT_B8G8R8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Layout: D3D12_TEXTURE_LAYOUT_UNKNOWN,
            Flags: D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET,
            ..Default::default()
        };
        let mut resource: Option<ID3D12Resource> = None;
        unsafe { self.device.CreateCommittedResource(&heap, D3D12_HEAP_FLAG_NONE, &desc, D3D12_RESOURCE_STATE_COMMON, None, &mut resource) }.ok()?;
        resource
    }
}

fn assert_red(surface: &mut Surface, info: &ImageInfo) {
    let image = surface.image_snapshot().unwrap();
    assert!(image.is_texture_backed());
    let mut pixels = vec![0u8; 16 * 16 * 4];
    let ok = unsafe { image.read_pixels(info, pixels.as_mut_ptr(), 16 * 4, 0, 0) };
    assert!(ok, "read_pixels failed");
    // ColorType::N32 is Bgra8888: opaque red is B=0, G=0, R=255, A=255.
    assert_eq!(&pixels[0..4], &[0, 0, 255, 255]);
    assert_eq!(&pixels[pixels.len() - 4..], &[0, 0, 255, 255]);
}

#[test]
fn render_target_round_trip() {
    let Some(fixture) = D3DFixture::new() else {
        eprintln!("skipping: no D3D12 hardware adapter available on this machine");
        return;
    };
    let mut context = fixture.direct_context().expect("DirectContext::new_d3d failed");

    let info = ImageInfo::new(16, 16, ColorType::N32, AlphaType::Premul);
    let mut surface = Surface::new_render_target(&mut context, false, &info, 0, SurfaceOrigin::TopLeft, None, false, false)
        .expect("failed to create GPU render-target surface");

    surface.canvas().clear(0xFFFF0000);
    context.flush();
    context.submit(true);
    assert_red(&mut surface, &info);
}

#[test]
fn wrap_backend_texture_round_trip() {
    let Some(fixture) = D3DFixture::new() else {
        eprintln!("skipping: no D3D12 hardware adapter available on this machine");
        return;
    };
    let mut context = fixture.direct_context().expect("DirectContext::new_d3d failed");
    let resource = fixture.create_texture(16, 16).expect("CreateCommittedResource failed");

    let backend_texture = BackendTexture::new_d3d(
        16,
        16,
        resource.as_raw(),
        D3D12_RESOURCE_STATE_COMMON.0 as u32,
        DXGI_FORMAT_B8G8R8A8_UNORM.0 as u32,
        1,
        1,
        0,
        false,
        "skialin-test-texture",
    )
    .expect("BackendTexture::new_d3d failed");
    assert!(backend_texture.is_valid());

    let mut surface = Surface::wrap_backend_texture(&mut context, &backend_texture, SurfaceOrigin::TopLeft, 0, ColorType::N32, None, None)
        .expect("wrap_backend_texture failed");

    surface.canvas().clear(0xFFFF0000);
    context.flush();
    context.submit(true);
    let info = ImageInfo::new(16, 16, ColorType::N32, AlphaType::Premul);
    assert_red(&mut surface, &info);

    // Skia AddRef'd the resource; dropping ours first must not free it out from under the surface.
    drop(resource);
    drop(surface);
    drop(backend_texture);
    context.flush();
    context.submit(true);
}
