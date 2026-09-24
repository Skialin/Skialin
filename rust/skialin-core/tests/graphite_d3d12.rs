//! Graphite + D3D12 (through Dawn) smoke test. Dawn creates the D3D12 device itself; the texture
//! import test creates its ID3D12Resource on that same device with the `windows` crate.
#![cfg(windows)]

use skialin_core::{AlphaType, ColorType, GraphiteContext, ImageInfo, Surface};
use windows::core::Interface;
use windows::Win32::Graphics::Direct3D12::{
    ID3D12Device, ID3D12Resource, D3D12_HEAP_FLAG_NONE, D3D12_HEAP_PROPERTIES, D3D12_HEAP_TYPE_DEFAULT, D3D12_RESOURCE_DESC,
    D3D12_RESOURCE_DIMENSION_TEXTURE2D, D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET, D3D12_RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS,
    D3D12_RESOURCE_STATE_COMMON, D3D12_TEXTURE_LAYOUT_UNKNOWN,
};
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_SAMPLE_DESC};

// wgpu::TextureFormat::BGRA8Unorm and wgpu::TextureUsage bits, from Dawn's generated webgpu.h.
const WGPU_TEXTURE_FORMAT_BGRA8_UNORM: u32 = 0x1B;
const WGPU_TEXTURE_USAGE_COPY_SRC: u32 = 0x01;
const WGPU_TEXTURE_USAGE_COPY_DST: u32 = 0x02;
const WGPU_TEXTURE_USAGE_TEXTURE_BINDING: u32 = 0x04;
const WGPU_TEXTURE_USAGE_RENDER_ATTACHMENT: u32 = 0x10;

#[test]
fn render_target_round_trip() {
    let Some(dawn) = GraphiteContext::new_dawn_d3d12(0) else {
        eprintln!("skipping: no D3D12 adapter available on this machine");
        return;
    };
    assert_ne!(dawn.d3d12_device, 0);
    assert_ne!(dawn.d3d12_command_queue, 0);
    let mut context = dawn.context;
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
fn import_d3d12_resource() {
    let Some(dawn) = GraphiteContext::new_dawn_d3d12(0) else {
        eprintln!("skipping: no D3D12 adapter available on this machine");
        return;
    };
    let mut context = dawn.context;
    let mut recorder = context.make_recorder().expect("make_recorder failed");

    // Borrowed, not owned: Dawn holds the device's only reference we're entitled to.
    let raw_device = dawn.d3d12_device as *mut std::ffi::c_void;
    let device = unsafe { ID3D12Device::from_raw_borrowed(&raw_device) }.expect("null ID3D12Device");
    let heap = D3D12_HEAP_PROPERTIES { Type: D3D12_HEAP_TYPE_DEFAULT, ..Default::default() };
    let desc = D3D12_RESOURCE_DESC {
        Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
        Width: 16,
        Height: 16,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: DXGI_FORMAT_B8G8R8A8_UNORM,
        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
        Layout: D3D12_TEXTURE_LAYOUT_UNKNOWN,
        // Dawn refuses to import a resource without ALLOW_SIMULTANEOUS_ACCESS.
        Flags: D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET | D3D12_RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS,
        ..Default::default()
    };
    let mut resource: Option<ID3D12Resource> = None;
    unsafe { device.CreateCommittedResource(&heap, D3D12_HEAP_FLAG_NONE, &desc, D3D12_RESOURCE_STATE_COMMON, None, &mut resource) }
        .expect("CreateCommittedResource failed");
    let resource = resource.unwrap();
    let backend_texture = context
        .make_d3d12_backend_texture(
            resource.as_raw() as i64,
            16,
            16,
            1,
            false,
            WGPU_TEXTURE_FORMAT_BGRA8_UNORM,
            WGPU_TEXTURE_USAGE_RENDER_ATTACHMENT | WGPU_TEXTURE_USAGE_TEXTURE_BINDING | WGPU_TEXTURE_USAGE_COPY_SRC | WGPU_TEXTURE_USAGE_COPY_DST,
        )
        .expect("make_d3d12_backend_texture failed");
    assert!(backend_texture.is_valid());
    let mut surface = Surface::wrap_graphite_backend_texture(&mut recorder, &backend_texture, ColorType::N32, None, None)
        .expect("wrap_graphite_backend_texture failed");
    assert_eq!(surface.width(), 16);
    surface.canvas().clear(0xFFFF0000);
    let mut recording = recorder.snap().expect("snap failed");
    let status = context.insert_recording(&mut recording, &mut surface);
    assert_eq!(status, 0, "insertRecording failed with status {status}");
    assert!(context.submit(true), "submit failed");

    // Tear down in dependency order: everything referencing the Dawn texture, then the texture's
    // keep-alive (inside backend_texture), then our resource -- which lives on Dawn's device, so it
    // has to go before the context that owns that device.
    drop(surface);
    drop(recording);
    drop(backend_texture);
    drop(resource);
    drop(recorder);
    drop(context);
}
